// We need to import library to TEST it
// Each file inside `tests` folders are separate `crate`.
// Cargo knows that all the files inside `tests` folders are tests; So, no need to use `#[test]`
// We can't directly use integration test for binary crates.
use ch11_testing_adder;

// looks for `common.rs` or `common > mod.rs`
mod common;

#[test]
fn it_adds_two_to_input() {
    assert_eq!(4, ch11_testing_adder::add_two(2));
}
