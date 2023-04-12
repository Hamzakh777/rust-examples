// ===== RefCell<T> and Interior mutability pattern
// - `interior mutability` is a design pattern in Rust that allows you to mutate the data even when
// there are immutable references to that data. The pattern uses `unsafe` code inside a data structure.

// We can use types that implement the interior mutability pattern when the borrowing rules will be followed at runtime
// even when the compiler can't guarantee that.

// The `unsafe` code involved is wrapped in a safe API, and the outer type is still immutable.

// === Enforcing borrowing rules at runtime with `RefCell<T>`
// `RefCell<T>` is useful when your sure your code follows the borrowing rules
// but the compiler is unable to to understand and guarantee that.
// Rust compiler by default is conservative, meaning it can reject a perfectly correct program
// if it doesn't understand it.

// INTERIOR MUTABILITY PATTERN: mutating the value inside an immutable value

// fn wrong_code() {
//     let x = 7;
//     let y = &mut x; // this wont compile as `x` is immutable
// }

// There are situations in which it would be useful for a value to mutate itself
// in its methods but appear immutable to other code.
// Code outside the value's methods would not be able to mutate the value.

use std::{rc::Rc, cell::RefCell};

// ===== Example mock object
// This trait is the interface our mock object need to implement so that  the mock be used 
// in the same way as the real object is.
// So if we want to implement a Mock, we need to use the trait and a custom struct that 
// implements that trait
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        Self {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgentd warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // struct MockMessenger {
    //     sent_messages: Vec<String>
    // }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // We cant modify the `MockMessenger` to keep track of the messages, 
            // because the `send` method takes an immutable reference to `self`
            // and we need a mutable reference
            // ===> A Situation where `interior mutability pattern` can help
            // self.sent_messages.push(String::from(msg));

            // We fix that by using `RefCell`
            // `borrow_mut` gives us a mutable reference to the value inside the `RefCell<Vec<String>>`
            // `borrow_mut` returns `RefMut<T>` which is a smart pointer, it implements the `Drop` trait
            // `RefCell` keep count of how many `RefMut<T>` we have
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // `borrow_mut` returns `Ref<T>` which is a smart pointer
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}


// ===== Combining `Rc<T>` and `RefCell<T>` to have multiple references to mutable data
// `Rc<T>` only lets us have multiple owners of immutable data
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn cons_list_rc_ref() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Mutex<T> is the thread-safe vesrion of `RefCell<T>`