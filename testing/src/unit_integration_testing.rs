// the convention for unit testing is to create a module under each file and annotate it with `#[cfg(test)]
// `cfg(test) tells rust to only compile the test module when you run `cargo test` not when you run `cargo build`
// `cfg` stands for configuration and tells rust that the following item should only be included in certain configuration options

// Rust allows us to test private functions unlike some other languages

// integration tests are put inside a `tests` directory.
