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

// How we specify lifetimes depend on what the function is doing
// Return value lifetime needs to be one of the parameters lifetimes
// fn longest2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
// // best way to fix this is to return an owned data type rather than a reference
//     result.as_str() // this returns a string slice (which is a reference), but result gets cleaned up at the end of the scope
//     // so it won't compile
// }

// we can also define structs to to hold references rather than owned data
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn use_struct_with_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

// ====== Lifetime Elision rules
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
// This is essentially what allows us in a lot of cases to not have to explicitly specify lifetimes
// In pre-1.0 rust every reference needed an explicit lifetime
// Lifetimes on function or method parameters are called `input lifetimes`
// Lifetimes on return values are called `output lifetimes`
// There are 3 rules to it that the compiler will follow, not the programmers
// - First applies to input lifetimes
// - The other two rules applies to the output lifetimes
// - First rule is that the compiler will assign a lifetime parameter to each parameter that's a reference
// - Second rule is if there is exactly one lifetime parameter it gets assigned to all output lifetime parameters
// - if one of the lifetime parameters is for `&self` or `&mut self` it gets assigned to all output lifetime parameters

// ====== Static lifetime
// 'static means the reference is meant to live for the entire duration of the program 
// All string literals have 'static lifetime
fn test() {
    let s: &'static str = "I have a static lifetime.";
}


pub fn demo() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
