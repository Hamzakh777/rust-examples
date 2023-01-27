fn main() {
    println!("Hello, world!");
    // there are two data types subsets: scalar and compound

    // ======= Scalar ==========
    // Rust has this type called 
    // usize and isize
    // it pretty much represents the computers architecture bits
    // if we are on 64bits, isize will be i64
    // this is useful if we are indexing some sort of collection.


    // floating points
    // all floating points in rust are signed
    // there are only two types f32 and f64, the default is f64 since on modern CPU the speed 
    // is nearly identical
    let float: f64 = 5.2;
    println!("my float {float}");

    // the character type
    // I think characters are different from strings
    // single quotes for chars and double quotes for strings 
    // Its 4 bytes in size 
    let char: char = 's';
    println!("my char {char}");

    // ========= Compound Types ==========
    // they can group multiple values into one type
    // Rust has two: tuples and arrays

    // tuples 
    let tup: (u32, char, i8) = (10, 'a', -2);
    let (a, b, c) = tup; // this is like destructuring in javascript
    let third_value = tup.2;
    println!("{a}, {third_value}");
    // unit is an empty tuple (), expression would implicitly return it if they don't return any other type
    let unit = ();


    // Array type
    // what's the difference between this and a tuple?
    // all the elements in the array must have the same type
    // and in Rust all array have fixed length
    // Arrays are useful when you want the data to be on the stack rather than the heap
    let example_array: [u8; 3] = [1, 3, 4];
}
