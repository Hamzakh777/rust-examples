use std::{io};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");
    
        // All rust variables by default are immutable
        // To make them mutable, we use `mut` keyword
        // `::` indicates that `new` is an associated function
        // of the `String` type
        // Associated function: is a function that is implemented on a type
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // switching from `expect` to `match` means instead of crashing the program,
        // we will have some error handling
        let guess: i32 = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("Your guess is: {guess}");
    
        // a `match` expression is made up of arms. An arm consists of a pattern to match against,
        // and the code that should run if the value passed to `match` fits the arms pattern
        // This is very similar to a switch
        // This is super useful because it lets you express a variaty of situation that my code can encounter 
        // and they make sure I handle them all.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // this is called shadowing. This is used a lot of converting a value from one type to another
}

// ======== BETTER ERROR HANDLING 
// We can create a custom type and add the validations
// in a function to create an instance of the type rather than 
// repeating the validation everywhere
pub struct Guess {
    // this is private by default
    // everyone will be forced to use Guess:new to create a new instance
    // hence enforcing the check
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
