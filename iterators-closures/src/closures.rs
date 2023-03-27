// Closures are anonymous functions that can be saved in a variable or passed as an argument to other functions
// Unlike functions, they capture values from the scope in which they're defined.

use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn run() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

// ===== Closure Type Inference and Annotation
// Closures are often short and only relevant in the context they are defined in
// Closures don't require you to annotate the types of the parameters or the return values like `fn`
// the compiler is able to infer the types of most variables (there are rare cases where it can't)
fn testing() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}

// ====== Capturing References or Moving ownership
// Closures can capture values from their environments in three ways, which directly map to the 
// three ways a function can take a parameter: 
// - borrowing immutably
// - borrowing mutably 
// - taking ownership
// The closure will decide which of these to use based on what the body of the function is doing with the captured values.
fn capturing() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // no immutable borrow of `list` between the closure definition and closure call, no other borrow is allowed
    // when there's a mutable borrow
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

// Use `move` to take ownership
fn closure_takes_ownership() {
    let list = vec![1, 2, 4];
    println!("Before {:?}", list);

    // If the main thread maintained ownership of list but ended before the new thread did and dropped list, 
    // the immutable reference in the thread would be invalid.
    thread::spawn(move || println!("from thread: {:?}", list)).join().unwrap();
}

// ====== Moving captured values out of closures and the `Fn` trait
// The way a closure captures and handles values from the environment affects which traits the 
// closures implements, and traits are how functions and structs can specify what kinds of closures they can use
// Refer here for more details https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits
// TL;DR (Fn traits: `FnOnce`, `FnMut` and `Fn`), all closures implement `FnOnce` and can only be called onc
// The `Fn` traits are important when defining or using functions or types that make use of closues