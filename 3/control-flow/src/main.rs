fn main() {
    // block of code associated with the conditions in if statement are sometimes called 
    // `arms`, similar to the case in `match`

    // Unlike other languages like Ruby and Javascript, 
    // Rust doesn't convert non-boolean types to boolean

    // In case you have a lot of `else if` statements,
    // use `match` instead

    // if is an expressions, so we can do this
    let condition = true;
    let number = if condition { 3 } else { 5 };

    // ========= Loops =========o
    // Rust has three kinds of loops `loop`, 'while` and 'for'

    // `loop`
    // it tells Rust to keep running a block of code forever until you tell it explicitly 
    // to stop
    // loop {
    //     println!("loop");
    // }
    // we can also return data from the loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    // If you have loops within loops, you can use loop labels to target which loop
    // when calling break or continue;
    'first_loop: loop {
        println!("first loop");
        'another_loop: loop {
            println!("second loop");
            break 'first_loop;
        }
    }

    // while
    // while the condition is true the code runs

    // For
    // used to loop through collections
    // We can use a while to do something similar, but its slower since you'd have 
    // to check the index condition on every loop.
    // more info here:  https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for
    // I think this is similar to a generator in javascript
    let a = [1, 3, 4, 6, 7];
    for item in a {
        println!("item {item}");
    }
    // for is used a lot even in cases where while might make sense simply because its faster and more secure
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


    println!("Counter value is {result}");
}
