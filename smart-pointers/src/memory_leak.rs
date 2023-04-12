// Memory leaks are Memory safe in Rust

use std::{cell::RefCell, rc::{Rc, Weak}, borrow::Borrow};

// A memory leak can happen if a reference cycle is created
// ==== Creating a reference cycle
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn reference_cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count((&a)));
    println!("a initial rc count {}", Rc::strong_count((&b)));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

// ===== Weak<T> to prevent reference cycles
#[derive(Debug)]
struct Node {
    value: i32,
    // adding a reference from the child to its parent
    // the pointer will be dropped even if Weak count is not 0, unlike 
    // To make sure the value still exists with Weak, we call the `upgrade` 
    // method which returns an `Option<T>` Thus Rust will force to handle both cases of 
    // reference existing or not
    // It can't be Rc because that will create a reference cycle
    // with `leas.parent` pointing to `branch` and `branch.children` pointing to `leaf`
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
// When you call Rc::downgrade, you get a smart pointer of type Weak<T>. 
// Instead of increasing the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count by 1.
// The Rc<T> type uses weak_count to keep track of how many Weak<T> references exist, similar to strong_count.  
// The difference is the weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.
pub fn prevent_reference_cycles() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

pub fn run() {
    reference_cycle();
    prevent_reference_cycles();
}