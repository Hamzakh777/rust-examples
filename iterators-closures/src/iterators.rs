// The iterator pattern allows you to perform some task on a sequence of items in turn.
// In Rust iterators are `lazy`, meaning they have no effect until you call the methods that consume the iterator

pub fn run() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // the `for` loop takes ownership of `v1_iter` and makes it mutable behind the scenes
    for item in v1_iter {
        println!("item {}", item);
    }
}

// ====== The `Iterator` Trait and the `next` method
// This code says, implementing the `Iterator` trait requires also defining the `Item` type
// In other words, the `Item` type will be returned from the iterator.
pub trait Iterator {
    // associated type -> chapter 19
    type Item;

    // Only one method required to be defined
    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

// ====== Methods that consume the iterator
// `Iterator` has many methods with default implementation provided by std
// - **consuming adaptors**: methods that call `next`, because calling them uses up the iterator.
fn consuming_adaptors() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    println!("total {}", total);
}

// ====== Methods that Produce other iterators
// - **iterator adaptors** are methods defined on the `Iterator` trait that don't consume the iterator.
// Instead they produce different iterators.
fn iterator_adaptors() {
    let v1 = vec![1, 2, 3];

    // `collect` consumes the iterator, and collects the resulting values into a collection data type
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
    println!("{:?}", v2);
}

// ===== Using Closures that Capture their environment
// Many **iterator adapters** take closures as arguments
// https://doc.rust-lang.org/stable/book/ch13-02-iterators.html?search=#using-closures-that-capture-their-environment

// ===== Choosing between iterators and loops 
// Most Rust programmers prefer to use the iterator style
// Iterators are faster than loops, they are one of Rusts `zero-cost abstractions`
// https://doc.rust-lang.org/stable/book/ch13-04-performance.html

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        // if you want to return owned values, call `into_iter`
        // if you want to iterate over mutable references, call `iter_mut`
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}
