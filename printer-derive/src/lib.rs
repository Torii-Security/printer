#![doc = include_str!("../README.md")]

use proc_macro_error::proc_macro_error;
use printer_core::printer_core;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn printer(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    printer_core(args.into(), input.into()).into()
}