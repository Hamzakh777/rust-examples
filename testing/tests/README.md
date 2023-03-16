Integration tests goes here, these are meant to test you libraries public API

Each file inside the `tests` directory is a separate crate, so we need to bring our library into each crate's scope

By default all the files under `tests` are treated as their own crate
To avoid that, use `tests/common/mod.rs` to declare a file that shouldn't be treated as an integration test

You can only have the integration tests for `library crates`, `binary crates` are not meant to expose functionality hence 
why no integration test support