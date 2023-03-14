use std::fmt::Display;

mod generics;
mod traits;
mod lifetimes;

// ==== Generic Type parameters, Trait bounds, Lifetimes
// This is the longest function from Listing 10-21 that returns the longer of two string slices. 
// But now it has an extra parameter named ann of the generic type T, 
// which can be filled in by any type that implements the Display trait as specified by the where clause. 
fn longest_with_an_annoucement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) ->  &'a str 
where 
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Traits and Traits bounds insures even though the types are generic, 
// they'll have the behavior the code needs.
// Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and 
// the generic type parameter T go in the same list inside the angle brackets after the function name.

fn main() {
    println!("Hello, world!");
    // We can combine Generics with Traits to constrain a generic type to 
    // accept only those types that have a particular behavior.
    generics::demo();
    traits::demo();
    lifetimes::demo();
}
