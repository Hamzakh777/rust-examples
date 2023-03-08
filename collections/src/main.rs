// ----- INTRO
// The Rust STD library includes a number of very useful data structures called `collections`
// they are stored on the heap
// The most common ones are:
// - Vector: like an array in js, allows us to store a number of different values
// - String: a collection of characters
// - Hashmap: This is essentially a key-value pair like an object in javascript
// There are other collection types that be found here: https://doc.rust-lang.org/std/collections/index.html

mod vectors;
mod strings;
mod hash_maps;

fn main() {
    println!("Starting program execution");
    vectors::testing_vectors();
    strings::testing_string();
    hash_maps::testing_hash_maps();
}

