use std::{fs::{File, self}, io::{ErrorKind, self, Read}};

pub fn panic() {
    // You can make rust print the call stack by using an environment variable on `!panic`
    // More details on unwinding vs aborting https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic
    // panic!("crash and burn");

    // Use Backtrace
    // The key to reading a Backtrace is to start from the top and read until you see the files you wrote
    // You need debug symbols enabled
    // let vec = vec![4,45,34];
    // vec[99];
}

pub fn result() {
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    let file_name = "hello.txt";
    let greeting_file_result = File::open(&file_name);
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(file) => file,
                Err(err) => panic!("File failed to be created {:?}", err),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    // You'll notice we have a lot of nested matches which is ugly to read. In Chapter 13 we'll learn about 
    // closures which will enable to achieve a similar behavior but with cleaner code.

    // The `Result` type has many methods that helps us do various and more specific tasks.
    // - unwrap => returns the value, usage is dangerous, fails with panic!
    // - expect => similar to unwrap, but allows us to pass a custom message to panic!

    // Propagating Errors
}


// propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// the pattern of propagating errors in rust is so common in Rust that Rust 
// provides the question mark operator `?` to make this easier.
// A Shortcut for Propagating Errors: the ? Operator
fn read_username_from_file_2() -> Result<String, io::Error> {
    // this is like javascript nullish coalescing operator
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new(); // this is the buffer
    username_file.read_to_string(&mut username)?;
    Ok(username)
    // There is a difference between what the match expression and what the ? operator does: 
    // error values that have the ? operator called on them go through the from function, 
    // defined in the From trait in the standard library, which is used to convert values from one type into another.
}

// we can even chain methods
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// reading a file into a string
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// the `?` operator can only be used on fuctions whos return type is compatible with the value  
// the `?` is used on.

// `?` can be used on both `Result` and `Option`