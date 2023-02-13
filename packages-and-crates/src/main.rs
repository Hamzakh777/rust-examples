mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn main() {
    
    // ===== Crates
    // They come in two forms:
    // ** Binary crates ** which are compiled and meant to be executed like a CLI application
    // ** Library creates ** similar to the rand library we used, these are meant to encapsulate functionality and share it across different projects

    // ===== Packages
    // A package is a bundle of one or more crates that provides a set of functionality. 
    // A package contains cargo.toml file that describes how to build those crates.
    // A package can contain as many binary crates as we like, but at most one library crate.
    // By default, src/main.rs means it a binary, src/lib.rs means it a library


    // ==== Module
    // - they are private by default
    // - we declare them by `mod name`
    // - we can also declare them by module.rs
    // - we can also declare them by module/main.rs
    // src/main.rs and src/lib.rs are called crate roots, the reason for that is the content of these two files form
    // a module called 
    // We make a module public or its extent by prefixing the declaration with the `pub` keyword

    // ==== Paths
    // They can take two forms
    // An absolute path: the full path starting from the crate root, you use `crate`
    // Relative path: using `self` or `super` or an identifier in the current module

    fn eat_at_restaurant() {
        // Absolute path 
        crate::front_of_house::hosting::add_to_waitlist();

        // relative path -- starting with a module name means that the path is relative
        front_of_house::hosting::add_to_waitlist();
    }
    // In rust all, all items (functions, methods, structs, enums, module and constants) are private to the parent module 
    // by default. This is a choice so implementation details are hidden by default.
    // Making the module public doesn't make it content public as well. 


}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        // by default, all Structs field inside a module are private
        // even if the struct is made public
        pub toast: String,
        seasonal_fruit: String,
    }

    // Because the Struct has a private field,
    // it need to provide a public associate function that 
    // allows us to create a full instance with this Struct (that even has the private field)
    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // If we make an enum public
    // all of its variants are public by default.
    pub enum Appetizer {
        Soup, 
        Salad,
    }

}