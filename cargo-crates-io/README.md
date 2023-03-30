## Release Profiles
Different options for compiling code. 
Cargo has two main profile: `dev` and `release`
By adding `[profile.*]` sections under the `Cargo.toml` you can customize any profile
More details [here](https://doc.rust-lang.org/cargo/reference/profiles.html)

## Publishing crates to crate.io
Details [here](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html)

#### Making useful Documentation comments
Rust has **documentation comments** that will generate **HTML documentation**. They use three slashes `///`
and support Markdown notation. This is only displayed for publish API items.
```
/// Adds one to one
///
/// # Examples
/// let args......
```
We generate the documentation comment by running `cargo doc`, it runs the `rustdoc` tool.
Commonly used sections inlude: `Examples`, `Panics`, `Errors` and `Safety`.

`cargo test` will run the code examples in your documentation as tests.

`//!` adds documentation to the item contains the comment not the item following the comment (like the crate itself)

#### Exporting a convenient public API with `pub use`
its essentially re-exporting deeply nested modules from the top level module, similar to js `export * from './[filehere]`.
The moduel structure of a crate is more relevant to the developers working on it compared to the ones consuming it.
```
pub use self::kinds::mix;

pub mod kinds {
    -- snip --
}

pub mod utils {
    -- snip --
}
```

## Cargo workspaces
A workspace is a set of packages that share the same `Cargo.toml` and output directory.

## Installing Binaries with `cargo install`
You can install and use binary crates locally. You can only install packages that have a binary target

## Extending cargo with custom commands
https://doc.rust-lang.org/stable/book/ch14-05-extending-cargo.html