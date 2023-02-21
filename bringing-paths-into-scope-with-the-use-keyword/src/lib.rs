// to declare modules as files
// We only need to load a file using a `mod` declaration once in our module tree.
// this is the Rust idiomatic way of doing things
mod example_module; // This is the part I was mising the last time when I switched to files and they weren't working
use example_module::example_module_internal;
use example_module::internal_2;

// the use keyword is pretty much like module exports in javascript. 
// You can import few small things without having to always bring the whole module
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// absolute path 
// use only creates a shortcut for the particular scope in which use is occurs
// use should be only used in its scope
use crate::front_of_house::hosting;

// we can re-export this module for others to consume by doing 
// pub use create::front_of_house::hosting;
// Re-exporting is useful when the internal struct of the code is 
// different than what the programmers calling my code would think of the domain.

// using nested paths to clean up large use lists
// -- INSTEAD OF --
// use std::io;
// use std::fmt::Result;
// -- WE DO --
// use std::{fmt::Result, io, self}; // self in this case refers to std

// if we want to bring all public items we do this
// use std::io::*;

// we can use the `as` keyword, stands for alias
use std::fmt::Result;
use std::io::Result as IoResult;

pub fn eat_at_restaurant() {

    // we could have imported the function on its own without having to use hosting::
    // but this is the idiomatic way, importing the module makes sure that the function 
    // we are calling is clearly not defined locally but belongs to that module
    hosting::add_to_waitlist();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
