use std::io;

fn main() {
    loop {
        println!("Please input your value");
        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Failed to read");
        // convert to number
        let value = match  value.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your value is {value}");
        let fibo: i32 = nth_number(value);
        println!("The fibonacci number is {fibo}");
        break;
    };
}

fn nth_number(number: i32) -> i32 {
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    };

    nth_number(number - 1) + nth_number(number - 2)
}
