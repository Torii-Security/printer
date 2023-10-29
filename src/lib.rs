#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// A macro to write test cases that might seem more familiar to mocha users.
/// It was designed to be used with ink! smart contracts (hence the name - printer), however it works for any test cases.
/// The whole idea revolves around having more concise code. 
/// There are four hooks available for the developers: `before_each`, `after_each`, `before_all` and `invariant`.
/// 
/// # Example
/// ```
/// #[cfg(test)]
/// #[printer]
/// mod tests {
///     #[before_each]
///     fn setup() {
///         println!("This will be printed at the beginning of each test case");
///     }
/// 
///     #[test]
///     fn sample_test() {
///         assert!(1 == 1);
///     }
/// 
///     #[test]
///     fn another_test() {
///         assert!(10 > 3);
///     }
/// }
/// ```
/// 
/// During compilation, the macro will expand and the resulting test case would be:
/// ```
/// #[test]
/// fn sample_test() {
///     println!("This will be printed at the beginning of each test case");
///     assert!(1 == 1);
/// }
/// 
/// #[test]
/// fn another_test() {
///     println!("This will be printed at the beginning of each test case");
///     assert!(10 > 3);
/// }
/// ```

pub use printer_derive::printer;
pub use tokio;
pub use crossbeam_channel;
pub use once_cell;