#![doc = include_str!("../README.md")]

mod tests;
mod utils;

use proc_macro2::TokenStream;
use proc_macro_error::abort;

use syn::{parse2, parse_quote, ItemFn, ItemMod, ItemStatic, ItemUse, Stmt};
use utils::{FunctionAttributeVariant, HooksData};

pub fn printer_core(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        abort!(args, "printer macro does not take any arguments");
    }

    let mut mod_item = match parse2::<ItemMod>(input.clone()) {
        Ok(item) => item,
        Err(_e) => abort!(input, "printer macro should be used on mod with tests"),
    };

    let hooks_data = get_hooks_from_mod(&mut mod_item);

    // removing the printer-related code so that the project will compile
    mod_item
        .content
        .as_mut()
        .unwrap()
        .1
        .retain(|member| match member {
            syn::Item::Fn(func) => {
                let func_attrs = &func.attrs;
                if let Some(attr) = func_attrs.get(0) {
                    match &attr.meta {
                        syn::Meta::Path(path) => {
                            let ident = path.segments[0].ident.to_string();
                            match FunctionAttributeVariant::from(ident) {
                                FunctionAttributeVariant::Printer(_) => false,
                                FunctionAttributeVariant::External => true,
                            }
                        }
                        _ => true,
                    }
                } else {
                    true
                }
            }
            _ => true,
        });

    let mut test_cases_num: usize = 0;
    let (_, item_vec) = mod_item.content.as_mut().unwrap();
    for mut item in item_vec {
        match &mut item {
            syn::Item::Fn(func) => {
                for attribute in &func.attrs.clone() {
                    match attribute.meta.clone() {
                        syn::Meta::Path(path) => {
                            let last = &path.segments[&path.segments.len() - 1].ident.to_string();
                            if last == "test" {
                                test_cases_num += 1;
                                let return_type_is_union = {
                                    match &func.sig.output {
                                        syn::ReturnType::Default => true,
                                        syn::ReturnType::Type(_, _) => false, // might need additional logic
                                    }
                                };

                                insert_code_end(func, &hooks_data.invariants, return_type_is_union);
                                insert_code_front(func, &hooks_data.invariants);

                                insert_code_front(func, &hooks_data.before_each);
                                insert_code_end(func, &hooks_data.after_each, return_type_is_union);

                                if !hooks_data.before_all.is_empty() {
                                    insert_syncing_code(func);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    // adding before_all
    if !hooks_data.before_all.is_empty() {
        let use_statement_crossbeam: ItemUse = parse_quote! {
            use printer::crossbeam_channel::{unbounded, Sender, Receiver};
        };
        let use_statement_once_cell: ItemUse = parse_quote! {
            use printer::once_cell::sync::OnceCell;
        };
        let use_statement_tokio: ItemUse = parse_quote! {
            use printer::tokio;
        };

        let static_sync_channel: ItemStatic = parse_quote! {
            static PRINTER_SYNC_CHANNEL: OnceCell<(Sender<bool>, Receiver<bool>)> = OnceCell::new();
        };

        // this requires tokio runtime
        let mut before_all_code: ItemFn = parse_quote! {
            #[tokio::test]
            async fn printer_before_all() {
                let (channel_s, _channel_recv) = PRINTER_SYNC_CHANNEL.get_or_init(|| unbounded());

                // before_all goes here...

                for _ in 0..#test_cases_num {
                    channel_s.send(true).unwrap();
                }
            }
        };

        for (idx, code_line) in hooks_data.before_all.iter().enumerate() {
            before_all_code
                .block
                .stmts
                .insert(1 + idx, code_line.clone());
        }

        mod_item
            .content
            .as_mut()
            .unwrap()
            .1
            .insert(0, before_all_code.into());
        mod_item
            .content
            .as_mut()
            .unwrap()
            .1
            .insert(0, static_sync_channel.into());
        mod_item
            .content
            .as_mut()
            .unwrap()
            .1
            .insert(0, use_statement_crossbeam.into());
        mod_item
            .content
            .as_mut()
            .unwrap()
            .1
            .insert(0, use_statement_tokio.into());
        mod_item
            .content
            .as_mut()
            .unwrap()
            .1
            .insert(0, use_statement_once_cell.into());
    }

    parse_quote! {
        #mod_item
    }
}

fn get_hooks_from_mod(mod_item: &mut ItemMod) -> HooksData {
    let (_, vec_items) = mod_item.content.clone().unwrap();

    let mut hooks_data = HooksData {
        before_each: vec![],
        after_each: vec![],
        invariants: vec![],
        before_all: vec![],
    };

    for item in vec_items {
        match item {
            syn::Item::Fn(func) => {
                let function_attrs = func.attrs;
                if let Some(attr) = function_attrs.get(0) {
                    match &attr.meta {
                        syn::Meta::Path(path) => {
                            let ident = path.segments[0].ident.to_string();
                            match FunctionAttributeVariant::from(ident) {
                                FunctionAttributeVariant::Printer(printer_variant) => {
                                    printer_variant
                                        .populate_hooks_data(&mut hooks_data, func.block.stmts);
                                }
                                FunctionAttributeVariant::External => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    hooks_data
}

fn insert_code_front(function: &mut ItemFn, code_blocks: &Vec<Vec<Stmt>>) {
    for code_block in code_blocks.iter().rev() {
        for code_line in code_block.iter().rev() {
            function.block.stmts.insert(0, code_line.clone());
        }
    }
}

fn insert_code_end(
    function: &mut ItemFn,
    code_blocks: &Vec<Vec<Stmt>>,
    return_type_is_union: bool,
) {
    for code_block in code_blocks.iter() {
        for code_line in code_block.iter() {
            if return_type_is_union {
                function.block.stmts.push(code_line.clone());
            } else {
                function
                    .block
                    .stmts
                    .insert(function.block.stmts.len() - 1, code_line.clone());
            }
        }
    }
}

fn insert_syncing_code(function: &mut ItemFn) {
    let syncing_code: Stmt = parse_quote! {
        while true {
            if let Some(channel) = PRINTER_SYNC_CHANNEL.get() {
                let this_receiver = channel.1.clone();
                let ready = this_receiver.recv().unwrap();
                assert!(ready);
                break;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
        }
    };

    function.block.stmts.insert(0, syncing_code);
}
