#![cfg(test)]

use crate::printer_core;
use quote::quote;

#[test]
fn ink_before_each_test() {
    let before = quote! {
        #[printer]
        #[cfg(all(test, feature = "e2e-tests"))]
        mod e2e_tests {
            use super::*;
            use ink_e2e::build_message;

            type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

            #[before_each]
            async fn setup() {
                let constructor = FlipperRef::new(false);
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;
            }

            #[ink_e2e::test]
            async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                // when
                let flip = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.flip());
                let _flip_res = client
                    .call(&ink_e2e::bob(), flip, 0, None)
                    .await
                    .expect("flip failed");

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), true));

                Ok(())
            }

            #[ink_e2e::test]
            async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                // given
                let constructor = FlipperRef::new_default();

                // when
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                Ok(())
            }
        }
    };

    let after = printer_core(quote!(), before);
    assert_eq!(
        after.to_string(),
        "# [printer] # [cfg (all (test , feature = \"e2e-tests\"))] mod e2e_tests { use super :: * ; use ink_e2e :: build_message ; type E2EResult < T > = std :: result :: Result < T , Box < dyn std :: error :: Error > > ; # [ink_e2e :: test] async fn it_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; let flip = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . flip ()) ; let _flip_res = client . call (& ink_e2e :: bob () , flip , 0 , None) . await . expect (\"flip failed\") ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , true)) ; Ok (()) } # [ink_e2e :: test] async fn default_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let constructor = FlipperRef :: new_default () ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: bob () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; Ok (()) } }"
    );
}

#[test]
fn ink_double_before_each_hook_single_function_test() {
    let before = quote! {
        #[printer]
        #[cfg(all(test, feature = "e2e-tests"))]
        mod e2e_tests {
            use super::*;
            use ink_e2e::build_message;

            type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

            #[before_each]
            async fn setup() {
                let constructor = FlipperRef::new(false);
            }

            #[before_each]
            async fn setup2() {
                let contract_acc_id = client
                .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;
            }

            #[ink_e2e::test]
            async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                // when
                let flip = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.flip());
                let _flip_res = client
                    .call(&ink_e2e::bob(), flip, 0, None)
                    .await
                    .expect("flip failed");

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), true));

                Ok(())
            }
        }
    };

    let after = printer_core(quote!(), before);
    assert_eq!(
        after.to_string(),
        "# [printer] # [cfg (all (test , feature = \"e2e-tests\"))] mod e2e_tests { use super :: * ; use ink_e2e :: build_message ; type E2EResult < T > = std :: result :: Result < T , Box < dyn std :: error :: Error > > ; # [ink_e2e :: test] async fn it_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; let flip = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . flip ()) ; let _flip_res = client . call (& ink_e2e :: bob () , flip , 0 , None) . await . expect (\"flip failed\") ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , true)) ; Ok (()) } }"
    )
}

#[test]
fn ink_before_each_test2() {
    let before = quote! {
        #[printer]
        #[cfg(all(test, feature = "e2e-tests"))]
        mod e2e_tests {
            use super::*;
            use ink_e2e::build_message;

            type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

            #[before_each]
            async fn setup() {
                let constructor = FlipperRef::new(false);

            }

            #[before_each]
            async fn setup2() {
                let contract_acc_id = client
                .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;
            }

            #[ink_e2e::test]
            async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                // when
                let flip = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.flip());
                let _flip_res = client
                    .call(&ink_e2e::bob(), flip, 0, None)
                    .await
                    .expect("flip failed");

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), true));

                Ok(())
            }
        }
    };

    let after = printer_core(quote!(), before);
    assert_eq!(
        after.to_string(),
        "# [printer] # [cfg (all (test , feature = \"e2e-tests\"))] mod e2e_tests { use super :: * ; use ink_e2e :: build_message ; type E2EResult < T > = std :: result :: Result < T , Box < dyn std :: error :: Error > > ; # [ink_e2e :: test] async fn it_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; let flip = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . flip ()) ; let _flip_res = client . call (& ink_e2e :: bob () , flip , 0 , None) . await . expect (\"flip failed\") ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , true)) ; Ok (()) } }"
    );
}

#[test]
fn ink_test_after_each() {
    let before = quote! {
        #[printer]
        #[cfg(all(test, feature = "e2e-tests"))]
        mod e2e_tests {
            use super::*;
            use ink_e2e::build_message;

            type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

            #[after_each]
            async fn final_assertion() {
                assert!(1 == 1);
                println!("After each test!");
            }

            #[ink_e2e::test]
            async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                let constructor = FlipperRef::new(false);
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                // when
                let flip = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.flip());
                let _flip_res = client
                    .call(&ink_e2e::bob(), flip, 0, None)
                    .await
                    .expect("flip failed");

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), true));

                Ok(())
            }

            #[ink_e2e::test]
            async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                // given
                let constructor = FlipperRef::new_default();

                // when
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                Ok(())
            }
        }
    };

    let after = printer_core(quote!(), before);
    assert_eq!(after.to_string(), "# [printer] # [cfg (all (test , feature = \"e2e-tests\"))] mod e2e_tests { use super :: * ; use ink_e2e :: build_message ; type E2EResult < T > = std :: result :: Result < T , Box < dyn std :: error :: Error > > ; # [ink_e2e :: test] async fn it_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; let flip = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . flip ()) ; let _flip_res = client . call (& ink_e2e :: bob () , flip , 0 , None) . await . expect (\"flip failed\") ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , true)) ; assert ! (1 == 1) ; println ! (\"After each test!\") ; Ok (()) } # [ink_e2e :: test] async fn default_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { let constructor = FlipperRef :: new_default () ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: bob () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; assert ! (1 == 1) ; println ! (\"After each test!\") ; Ok (()) } }");
}

#[test]
fn ink_test_invariant_attr() {
    let before = quote! {
        #[printer]
        #[cfg(all(test, feature = "e2e-tests"))]
        mod e2e_tests {
            use super::*;
            use ink_e2e::build_message;

            type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

            #[invariant]
            async fn always_holds() {
                assert!(1 == 1);
                println!("In front and at the end!");
            }

            #[ink_e2e::test]
            async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                let constructor = FlipperRef::new(false);
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                // when
                let flip = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.flip());
                let _flip_res = client
                    .call(&ink_e2e::bob(), flip, 0, None)
                    .await
                    .expect("flip failed");

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), true));

                Ok(())
            }

            #[ink_e2e::test]
            async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                // given
                let constructor = FlipperRef::new_default();

                // when
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                Ok(())
            }
        }
    };

    let after = printer_core(quote!(), before);
    assert_eq!(after.to_string(), "# [printer] # [cfg (all (test , feature = \"e2e-tests\"))] mod e2e_tests { use super :: * ; use ink_e2e :: build_message ; type E2EResult < T > = std :: result :: Result < T , Box < dyn std :: error :: Error > > ; # [ink_e2e :: test] async fn it_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { assert ! (1 == 1) ; println ! (\"In front and at the end!\") ; let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; let flip = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . flip ()) ; let _flip_res = client . call (& ink_e2e :: bob () , flip , 0 , None) . await . expect (\"flip failed\") ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , true)) ; assert ! (1 == 1) ; println ! (\"In front and at the end!\") ; Ok (()) } # [ink_e2e :: test] async fn default_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { assert ! (1 == 1) ; println ! (\"In front and at the end!\") ; let constructor = FlipperRef :: new_default () ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: bob () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; assert ! (1 == 1) ; println ! (\"In front and at the end!\") ; Ok (()) } }");
}

#[test]
fn ink_before_each_after_each_invariant_test() {
    let before = quote! {
        #[cfg(all(test, feature = "e2e-tests"))]
        #[printer]
        mod e2e_tests {
            use super::*;
            use ink_e2e::build_message;

            type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

            #[before_each]
            async fn setup() {
                let constructor = FlipperRef::new(false);
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;
                println!("Only at the beginning");
            }

            #[after_each]
            fn cleanup() {
                println!("At the end only");
            }

            #[invariant]
            fn always_holds() {
                println!("this goes in the front...");
                assert!(1 == 1);
                println!("...and at the end!");
            }

            #[ink_e2e::test]
            async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                // when
                let flip = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.flip());
                let _flip_res = client
                    .call(&ink_e2e::bob(), flip, 0, None)
                    .await
                    .expect("flip failed");

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), true));

                Ok(())
            }

            #[ink_e2e::test]
            async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                // when
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                Ok(())
            }
        }
    };

    let after = printer_core(quote!(), before);
    assert_eq!(after.to_string(), "# [cfg (all (test , feature = \"e2e-tests\"))] # [printer] mod e2e_tests { use super :: * ; use ink_e2e :: build_message ; type E2EResult < T > = std :: result :: Result < T , Box < dyn std :: error :: Error > > ; # [ink_e2e :: test] async fn it_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; println ! (\"Only at the beginning\") ; println ! (\"this goes in the front...\") ; assert ! (1 == 1) ; println ! (\"...and at the end!\") ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; let flip = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . flip ()) ; let _flip_res = client . call (& ink_e2e :: bob () , flip , 0 , None) . await . expect (\"flip failed\") ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , true)) ; println ! (\"this goes in the front...\") ; assert ! (1 == 1) ; println ! (\"...and at the end!\") ; println ! (\"At the end only\") ; Ok (()) } # [ink_e2e :: test] async fn default_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; println ! (\"Only at the beginning\") ; println ! (\"this goes in the front...\") ; assert ! (1 == 1) ; println ! (\"...and at the end!\") ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: bob () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; println ! (\"this goes in the front...\") ; assert ! (1 == 1) ; println ! (\"...and at the end!\") ; println ! (\"At the end only\") ; Ok (()) } }");
}

#[test]
fn regular_test_mod() {
    let before = quote! {
        #[cfg(test)]
        #[printer]
        mod tests {
            #[before_each]
            fn setup() {
                println!("This goes first!");
                let c = 12;
            }

            #[after_each]
            fn wrap() {
                assert!(a + b + c > 5);
            }

            #[test]
            fn sample_test() {
                let a = 1;
                let b = 10;
                assert!(a + b == 11);
            }
        }
    };

    let after = printer_core(quote!(), before);
    assert_eq!(after.to_string(), "# [cfg (test)] # [printer] mod tests { # [test] fn sample_test () { println ! (\"This goes first!\") ; let c = 12 ; let a = 1 ; let b = 10 ; assert ! (a + b == 11) ; assert ! (a + b + c > 5) ; } }");
}

#[test]
fn ink_before_all_test() {
    let before = quote! {
    #[printer]
        #[cfg(all(test, feature = "e2e-tests"))]
        mod e2e_tests {
            use super::*;
            use ink_e2e::build_message;

            type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

            #[before_all]
            async fn first() {
                println!("This should be executed first!");
            }

            #[before_each]
            async fn setup() {
                let constructor = FlipperRef::new(false);
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;
            }

            #[ink_e2e::test]
            async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                // when
                let flip = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.flip());
                let _flip_res = client
                    .call(&ink_e2e::bob(), flip, 0, None)
                    .await
                    .expect("flip failed");

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), true));

                Ok(())
            }

            #[ink_e2e::test]
            async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                // given
                let constructor = FlipperRef::new_default();

                // when
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                Ok(())
            }
        }
    };

    let after = printer_core(quote!(), before);
    assert_eq!(after.to_string(), "# [printer] # [cfg (all (test , feature = \"e2e-tests\"))] mod e2e_tests { use printer :: once_cell :: sync :: OnceCell ; use printer :: tokio ; use printer :: crossbeam_channel :: { unbounded , Sender , Receiver } ; static PRINTER_SYNC_CHANNEL : OnceCell < (Sender < bool > , Receiver < bool >) > = OnceCell :: new () ; # [tokio :: test] async fn printer_before_all () { let (channel_s , _channel_recv) = PRINTER_SYNC_CHANNEL . get_or_init (| | unbounded ()) ; println ! (\"This should be executed first!\") ; for _ in 0 .. 2usize { channel_s . send (true) . unwrap () ; } } use super :: * ; use ink_e2e :: build_message ; type E2EResult < T > = std :: result :: Result < T , Box < dyn std :: error :: Error > > ; # [ink_e2e :: test] async fn it_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { while true { if let Some (channel) = PRINTER_SYNC_CHANNEL . get () { let this_receiver = channel . 1 . clone () ; let ready = this_receiver . recv () . unwrap () ; assert ! (ready) ; break ; } else { std :: thread :: sleep (std :: time :: Duration :: from_millis (200)) ; } } let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; let flip = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . flip ()) ; let _flip_res = client . call (& ink_e2e :: bob () , flip , 0 , None) . await . expect (\"flip failed\") ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , true)) ; Ok (()) } # [ink_e2e :: test] async fn default_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { while true { if let Some (channel) = PRINTER_SYNC_CHANNEL . get () { let this_receiver = channel . 1 . clone () ; let ready = this_receiver . recv () . unwrap () ; assert ! (ready) ; break ; } else { std :: thread :: sleep (std :: time :: Duration :: from_millis (200)) ; } } let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let constructor = FlipperRef :: new_default () ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: bob () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; Ok (()) } }");
}

#[test]
#[should_panic]
fn printer_macro_used_on_non_mod() {
    let before = quote! {
        #[printer]
        pub fn something() {}
    };

    let _after = printer_core(quote!(), before);
}

#[test]
#[should_panic]
fn printer_macro_with_args() {
    let before = quote! {
        #[cfg(test)]
        #[printer(argument)]
        mod tests {
            #[before_each]
            fn setup() {
                println!("something");
            }

            #[test]
            fn sample() {
                assert!(1 == 1);
            }
        }
    };

    let _after = printer_core(quote!("argument"), before);
}

#[test]
fn printer_with_helper_functions_in_tests_mod() {
    let before = quote! {
    #[printer]
        #[cfg(all(test, feature = "e2e-tests"))]
        mod e2e_tests {
            use super::*;
            use ink_e2e::build_message;

            type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

            #[before_all]
            async fn first() {
                println!("This should be executed first!");
            }

            #[before_each]
            async fn setup() {
                let constructor = FlipperRef::new(false);
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;
            }

            async fn helper_function() {
                println!("Doing something here");
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            #[ink_e2e::test]
            async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                // when
                let flip = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.flip());
                let _flip_res = client
                    .call(&ink_e2e::bob(), flip, 0, None)
                    .await
                    .expect("flip failed");

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), true));
                helper_function.await;

                Ok(())
            }

            #[ink_e2e::test]
            async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
                // given
                let constructor = FlipperRef::new_default();

                // when
                let contract_acc_id = client
                    .instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
                    .await
                    .expect("instantiate failed")
                    .account_id;

                // then
                let get = build_message::<FlipperRef>(contract_acc_id.clone())
                    .call(|flipper| flipper.get());
                let get_res = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
                assert!(matches!(get_res.return_value(), false));

                Ok(())
            }
        }
    };

    let after = printer_core(quote!(), before);
    assert_eq!(after.to_string(), "# [printer] # [cfg (all (test , feature = \"e2e-tests\"))] mod e2e_tests { use printer :: once_cell :: sync :: OnceCell ; use printer :: tokio ; use printer :: crossbeam_channel :: { unbounded , Sender , Receiver } ; static PRINTER_SYNC_CHANNEL : OnceCell < (Sender < bool > , Receiver < bool >) > = OnceCell :: new () ; # [tokio :: test] async fn printer_before_all () { let (channel_s , _channel_recv) = PRINTER_SYNC_CHANNEL . get_or_init (| | unbounded ()) ; println ! (\"This should be executed first!\") ; for _ in 0 .. 2usize { channel_s . send (true) . unwrap () ; } } use super :: * ; use ink_e2e :: build_message ; type E2EResult < T > = std :: result :: Result < T , Box < dyn std :: error :: Error > > ; async fn helper_function () { println ! (\"Doing something here\") ; std :: thread :: sleep (std :: time :: Duration :: from_millis (100)) ; } # [ink_e2e :: test] async fn it_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { while true { if let Some (channel) = PRINTER_SYNC_CHANNEL . get () { let this_receiver = channel . 1 . clone () ; let ready = this_receiver . recv () . unwrap () ; assert ! (ready) ; break ; } else { std :: thread :: sleep (std :: time :: Duration :: from_millis (200)) ; } } let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; let flip = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . flip ()) ; let _flip_res = client . call (& ink_e2e :: bob () , flip , 0 , None) . await . expect (\"flip failed\") ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , true)) ; helper_function . await ; Ok (()) } # [ink_e2e :: test] async fn default_works (mut client : ink_e2e :: Client < C , E >) -> E2EResult < () > { while true { if let Some (channel) = PRINTER_SYNC_CHANNEL . get () { let this_receiver = channel . 1 . clone () ; let ready = this_receiver . recv () . unwrap () ; assert ! (ready) ; break ; } else { std :: thread :: sleep (std :: time :: Duration :: from_millis (200)) ; } } let constructor = FlipperRef :: new (false) ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: alice () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let constructor = FlipperRef :: new_default () ; let contract_acc_id = client . instantiate (\"flipper\" , & ink_e2e :: bob () , constructor , 0 , None) . await . expect (\"instantiate failed\") . account_id ; let get = build_message :: < FlipperRef > (contract_acc_id . clone ()) . call (| flipper | flipper . get ()) ; let get_res = client . call_dry_run (& ink_e2e :: bob () , & get , 0 , None) . await ; assert ! (matches ! (get_res . return_value () , false)) ; Ok (()) } }");
}

