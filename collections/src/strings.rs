pub fn testing_string() {
    // this is a string literal - they are encoded into the actual byte code of the program
    let data = "initial contents";

    // Strings are implemented as collection of bytes
    // String is not included in Rust Core, its part of the STD

    // String is actually implemented as a wrapper around a vector of bytes with some extra guarantees
    let s2 = data.to_string(); // to_string is available on any type that implements the Display trait
    // or any string literal
    let s3 = "emow".to_string();
    let mut s = String::new();

    // A String can grow in size and its content can change just like a Vec<T>
    s.push_str("meow meow");
    // push_str doesn't take ownership, it takes a string slice &str
    
    // we can add a single character using push
    s.push('d')

    // we can concatenate two strings
    let concatenated = s3 + &s2; // s3 has been moved here and can no longer be used
    // the + operator use a method called add which signature looks like this fn add(self, s: &str) -> String {}
    // check the chapter again to understand this better https://doc.rust-lang.org/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro

    // for a more complicated string combining we can use the `!format` macro
    let a1 = String::from("tic");
    let a2 = String::from("tac");
    let a3 = String::from("toe");

    let a = format!("{a1}-{a2}-{a3}"); // `format!` uses references so this call doesn't take ownership of any of the parameters

    // Indexing into strings
    // this will give you an error 
    // let h = a2[0]
    // It doesnt work because UTF range is more than the range we can get by 1 byte (2**7)
    // as a result, sometimes we need two bytes to represent a single character 
    // Unicode scalar value === rusts `char` type
    // Another issue with indexing a string is that indexing operations are expected to always 
    // take constant time (O(1))
    // indexing into a string is a bad idea because its not clear what rust should return:
    // a byte value, a character or a grapheme cluster or a string slice

    // Slicing strings
    let sliced = &a3[0..2];
    // you should use ranges to create string slices with caution, because it can crash the program 

    // Methods of iterating over a string
    // the best way to operate on a piece of String is to be explicit wether you want characters or bytes
    // 1. For individual Unicode scalar values, use the `chars` method
    for c in "Зд".chars() {
        println!("{c}");
    }
    // 2. The `bytes` method returns the raw bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // REMEMBERS
    // ===========
    // Unicode Scalar values can be made up of more than 1 byte.

    // There are other methods like `contains` and `replace` which are implemented under the String type

}