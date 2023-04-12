// Box<T> allows you to store data on the heap instead of the stack
// Use cases
// - Recursive Types (example below)
// - Using a type who size is not known at compile time in the context of a type that requires an exact size
// - When you have large amounts of data and you want to transfer ownership but ensure the data don't be copied when you do so
// - When you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type

// Indirection: Instead of storing the value directly, we should change the data structure to store the value indirectly 
// by storing a pointer to the value instead

// Rust always need to know the size of the enum
// Size of enum = size of the largest variant
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// This is documentation
pub fn run() {
    let list =  List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}