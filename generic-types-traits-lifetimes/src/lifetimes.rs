// Lifetimes ensure that references are valid as long as we need them to be
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

// Rust needs to know which references will be returned
// to fix this will use `generic lifetime parameters` that define the relationship between the references
// so the borrow checker can perform its analysis
// fn longest(x: &str, y: &str) -> &str {
    // x and y have different references so rust at compile time doesn't know which one should be returned
//     if x.len() > y.len() {
//     } else {
//         y
//     }
// }

// Lifetime annotation syntax
// &i32 -- a reference with an implicit lifetime
// &'a i32 -- a reference with an explicit lifetime
// &'a mut i32 -- a mutable reference with an explicit lifetime
// One lifetime annotations by itself doesn't have much meaning, 
// because they are meant to tell Rust how generic lifetime parameters 
// of multiple references relate to each other

// == Lifetime annotations in function signature 
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures
// When annotating lifetimes in functions, the annotations go in the function signature
// Remember, when specify the lifetime signature, we're not changing the lifetime of any values passed in 
// or returned. Rather, we're specifying that the borrow checker should reject in value 
// that doesn't adhere to this requirement
// When we pass concrete references to longest, 
// the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y.
// 'a will get the smaller lifetime of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn demo() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
