// Rc<T>
// There are cases where we want multiple owner to own a value
// It only allows immutable references => Read only. You can use the **interior mutability pattern** and the `RefCell<T>` with `Rc<T>` to work with this immutability restriction.
// Example: Graph data structures
// Rc<T> keeps track of the number of references to a value to determine
// wether or not the value is still in use.
// Rc<T> is only used for single threaded scenarios

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use List::{Cons, Nil};

// fn wrong_way_to_do_it() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     // this code wont' work because the ownership of `a` is moved into `b`
//     let b = Cons(3, Box::new(a));
//     // so `a` is no longer in scope here
//     // we can solve this problem by using `lifetime parameters`, but that will mean
//     // that every element in the list will at least as long as the entire list
//     let c = Cons(4, Box::new(a));
// }

fn run() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // each time we use `Rc:clone` the reference count increases
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // when the reference count is 0 the value is dropped

    // We clould have called `a.clone` but that makes a deep copy (they can take a lot of time)
    // `Rc::clone` doesn't make a deep copy and its the convention in Rust

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc() {}
}

