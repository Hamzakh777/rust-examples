fn main() {
    println!("Hello, world!");
    // the if let syntax lets us combine `if` and `let` into a less verbose way 
    // to handle values that match one expression and ignores the rest

    // instead of this 
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("the number is {max}"),
        _ => (),
    }

    // we could write 
    // the first part is the pattern 
    // and the second is the value
    if let Some(max) = config_max {
        println!("The number is {max}");
    }

    // when using `if let` we lose the exhaustive nature of match

    // we also add an else statement to it
    // for example, using the previous coins example,
    // if we want to count all the none quarter coins we'd do that in the else block
}
