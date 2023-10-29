# Printer

Printer is made to be a convenience tool assisting with writing tests in Rust. It's primary target is for testing smart contracts written in ink! (hence the name - printer), however it works with all tests. 

The main idea behind it revolves around hooks that should be familiar to developers using Mocha for testing (see testing smart contracts in hardhat).

Currently there are 4 hooks implemented:

- `before_each` : code marked with this hook will be executed at the beginning of each test case (just before the code written in the test itself).
- `after_each` : code marked with this hook will be executed at the end of each test case (just after the code written in the test itself).
- `before_all` : code marked with this hook will be executed before any other test case. Internally, it creates another test case that will be executed and it allows other test cases to run after it finishes (via a crossbeam channel)
- `invariant` : code marked with this hook is similar to using `before_each` and `after_each` with the same code. However, if those hooks are used with the `invariant`, the `invariant` code will be executed always after `before_each` and before `after_each`

## Usage

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
printer = "TODO"
```

## Examples

To keep things simple, here is the example based on the Flipper contract (template generated via `cargo contract new`):

```rust
#[cfg(all(test, feature = "e2e-tests"))]
#[printer::printer]
mod e2e_tests {
    use super::*;
    use ink_e2e::build_message;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[before_all]
    async fn this_goes_first() {
        println!("IN BEFORE ALL. WAITING JUST FOR KICKS");
        std::thread::sleep(std::time::Duration::from_secs(10));
        println!("THE TESTS SHOULD RUN NOW");
    }

    #[before_each]
    async fn setup() {
        let constructor = FlipperRef::new(false);
        let contract_account_id = client
            .instantiate("printer_test2", &ink_e2e::bob(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;
        println!("Completed setup!");
    }

    #[ink_e2e::test]
    async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let get = build_message::<FlipperRef>(contract_account_id.clone())
            .call(|printer_test2| printer_test2.get());
        let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
        assert!(matches!(get_result.return_value(), false));

        // When
        let flip = build_message::<FlipperRef>(contract_account_id.clone())
            .call(|printer_test2| printer_test2.flip());
        let _flip_result = client
            .call(&ink_e2e::bob(), flip, 0, None)
            .await
            .expect("flip failed");

        // Then
        let get = build_message::<FlipperRef>(contract_account_id.clone())
            .call(|printer_test2| printer_test2.get());
        let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
        assert!(matches!(get_result.return_value(), true));

        Ok(())
    }

    #[ink_e2e::test]
    async fn it_works2(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        println!("Second it works, adding a sleep for variety");
        std::thread::sleep(std::time::Duration::from_secs(3));
        
        let get = build_message::<FlipperRef>(contract_account_id.clone())
            .call(|printer_test2| printer_test2.get());
        let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
        assert!(matches!(get_result.return_value(), false));

        // When
        let flip = build_message::<FlipperRef>(contract_account_id.clone())
            .call(|printer_test2| printer_test2.flip());
        let _flip_result = client
            .call(&ink_e2e::bob(), flip, 0, None)
            .await
            .expect("flip failed");

        // Then
        let get = build_message::<FlipperRef>(contract_account_id.clone())
            .call(|printer_test2| printer_test2.get());
        let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
        assert!(matches!(get_result.return_value(), true));

        Ok(())
    }
}
```

In the above example both test cases do the same thing. They are there to showcase the `printer`'s capabilities.

## Notes

It does not matter what is the signature of the function marked with a hook - the way `printer` is implemented is that code present in those funcitons is copied and inserted in appropriate places in the test cases. Hence, for instance, you'll have access to variables created in the `before_each` hook in the test case. Similarly, the code in `after_each` will have access to variables created in the testcases. Furthermore, being most relevant to `before_each` - you have access to testcases parameters (see Flipper example and `client` variable).

There is an exception. Namely, the `before_all` hook will be converted to the separate test case. The `printer` will make sure that `before_all` code will be executed before any other test case starts executing their logic. Currently, it is not possible to access variables defined in the `before_all` hook - it is planned to be implemented in the future. Hence, the `before_all` hook is the most relevant in blockchain environment, when you need to request an airdrop, or setup external contracts that do not preserve relevant state or it does not matter if they do. Note that, currently `before_all` will be executed as a regular `tokio::test`, to maintain compatibility with non-ink! projects, i.e. you don't have access to the `ink_e2e::Client` there. Also, the `printer` reexports tokio crate (and other dependencies it uses) and adds appropriate `use` statements to the test cases, so you don't need to use `tokio::test`s yourself in order to benefit from `before_all`.
