// running `cargo test --help` shows the options passed to the compiler
// running `cargo test -- --help` show the options passed to the binary

// Tests by default are run in parallel but you can make them run consecutively
// to run them consecutively `cargo test -- --threads=1`
// `cargo test -- --show-output` shows the printed values even for passing tests

// pass the name of the function to run just one tests `cargo test testing_one_function`
// we can run multiple tests that match a keyword `cargo test testing`

// if we want to run only the ignored tests we can use `cargo test -- --ignored`

// if you want to run all tests ignored or not `cargo test -- --include-ignored`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_one_function() {
        assert!(true)
    }

    #[test]
    fn testing_two() {
        assert!(true)
    }

    #[ignore = "i dont like this test"]
    fn ignore_this_one() {
        assert!(true)
    }
}