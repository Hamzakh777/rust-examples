mod generics;
mod traits;
mod lifetimes;

fn main() {
    println!("Hello, world!");
    // We can combine Generics with Traits to constrain a generic type to 
    // accept only those types that have a particular behavior.
    generics::demo();
    traits::demo();
    lifetimes::demo();
}
