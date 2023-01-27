fn main() {
    println!("Hello, world!");

    // Rust doesn't care where you define your functions, only that 
    // they're defined somewhere in a scope that can be seen by the caller
    another_function();

    // Statements and expressions
    // function bodies are made of a series of statements optionally ending with an expression
    // Statement = instructions that perform some action and don't return any value
    // Expressions = evaluate to a resultant value

    // let x = (let y = 5); will result in an error, because (let y = 5) is a statement and doesn't return anything.
    // Statement doesn't return a value
    // In a lot of other languages (javascript included), (let y = 5) is an expression and thus will return a value.
    // A scope block is an expression
    let y = {
        let x = 4;
        x + 3 // note that we explicitly don't have semicolon at the end
    }
    // The 6 in (let y = 6) is an expression that evaluates to the value 6
    // calling a function is an expression
}

fn another_function() {
    println!("another function!");
}
