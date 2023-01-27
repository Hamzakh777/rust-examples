fn main() {
    // by default all Rust variables are immutable
    // we can make the variable mutable by adding the `mut` keyword
    let mut x = 5;
    println!("Hello, world! {x}");
    x = 7;
    println!("hello {x}");

    // Constants can be declared in any scope, which makes them very useful
    // for values that many parts of our code need to know about
    // they can only be set to a value of a constant expression, not a
    // value calculated a runtime
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;

    // Shadowing
    // the different between shadowing and mut is that we can change the type of the variable 
    // when using shadowing
    // also we'll get a compiler error if we try to reassign to this variable without the let keyword
    let y = 2;
    let y = y + 1; 
    println!("y value is {y}");

    {
        let y = y * 2; 
        println!("y value is {y}");
    }
    println!("y value is {y}");
}
