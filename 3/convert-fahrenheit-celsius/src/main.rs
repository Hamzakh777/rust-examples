use std::io;

fn main() {
    loop {
        println!("Please input a number of fahrenheit");
        // a function to take an input
        let mut fahrenheit: String = String::new();
        // read the input
        io::stdin().read_line(&mut fahrenheit).expect("Failed to read");
        // convert the input to a number
        let fahrenheit = match fahrenheit.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("Fahrenheit: {fahrenheit}");
        let celsius = (fahrenheit - 32) * 5/9;
        println!("Celsius {celsius}");
        break;
    };
}
