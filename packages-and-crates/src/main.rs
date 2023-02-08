fn main() {
    
    // ===== Crates
    // They come in two forms:
    // ** Binary crates ** which are compiled and meant to be executed like a CLI application
    // ** Library creates ** similar to the rand library we used, these are meant to encapsulate functionality and share it across different projects

    // ===== Packages
    // A package is a bundle of one or more crates that provides a set of functionality. 
    // A package contains cargo.toml file that describes how to build those crates.
    // A package can contain as many binary crates as we like, but at most one library crate.
    // By default, src/main.rs means it a binary, lib/main.rs means it a library
}
