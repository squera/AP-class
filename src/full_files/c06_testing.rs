
// open the other project    `libtest`

// There, you'll find a TEST
// A test in Rust is a function that’s annotated with the test attribute.
// Attributes are metadata about pieces of Rust code; one example is the derive attribute we used with structs in Chapter 5.
// To change a function into a test function, add #[test] on the line before fn.
// When you run your tests with the
//      cargo test
// command, Rust builds a test runner binary that runs the functions annotated with the test attribute and reports on whether each test function passes or fails.
//
// When we make a new library project with Cargo, a test module with a test function in it is automatically generated for us.
// This module helps you start writing your tests so you don’t have to look up the exact structure and syntax of test functions every time you start a new project.
// Previously we've had you all write unit tests.
// Unit tests are testing one module in isolation at a time:
// they're small and can test private code.
// This is in contrast to integration tests, which are external to your crate and use only its public interface in the same way any other code would.
// Their purpose is to test that many parts of your library work correctly together.