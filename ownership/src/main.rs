fn main() {
    // All of the following is applicable to all complex types that don't 
    // have a fixed size at compile time
    println!("Hello, world!");

    // The String type.
    // Its a complex type and stored on the heap because it has unknown size
    let s = String::from("hello");  
    // :: allows us to namespace the `from` function under String
    // it can be mutated 
    let mut s = String::from("hello");
    s.push_str(", meow");
    println!("s {s}");

    // In traditional programming languages we need to explicitly `allocate` and `free` memory
    // In Rust, the memory is automatically returned once the variable that owns the value goes out of scope
    {
        let a = String::from("adf");
        // a if valid from this point onward

        // do stuff with a
    };
    // a is no longer valid, scope is over
    // Rust calls a special function to do this and its `drop`

    // when doing 
    let s1 = String::from("asdf");
    let s2 = s1;
    // Rust doesn't copy the data over but simply copies the pointer that points to the heap location
    // If rust were to copy the data on the heap as well this operation would be very expensive.
    // more details can be found here https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // Rust fixes this issue by considering s1 no longer valid
    // println!("s1 {s1}");
    // will cause an error;
    // This is a `move` instead of `shallow copy`

    // we can actually copy the string by calling the method s1.clone()


    // Onwership and functions
    let meow = String::from("meow");
    take_ownership(meow);
    // we no longer can use meow here

    let number = 3;
    makes_copy(number);

    // we still can use number here;
}
// here, number goes out of scope then meow. But since meow was moved, nothing special happens

fn take_ownership(string_var: String) { // Here string_var comes into scope
    // do stuff with string_var\
    println!("take ownership {string_var}");
}
// string_var goes out of scope and `drop` is called 

fn makes_copy(number_var: i32) { // number_var comes into scope
 // do stuff with number_var
 println!("makes copy {number_var}");
}
// number_var goes out scope and nothing happens since its copied (on the stack)


// A conclusion around the borrowing concept
// For complex types, they are moved by default. Passing an argument is an expression, so that pretty much like creating a new variable
// that points to the same heap location.
// For simple types that live on the stack, they are copied.
// The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it

// Return values and scope
fn return_values_and_scope() {
    // return values can also transfer ownership
}