/// There are two kinds of macros
/// - DECLARATIVE MACROS: like `println!` (the most popular in Rust) (AKA: macros by example, macro_rules! macros or plain macros)
/// - PROCEDURAL_MACROS: three kinds 
///     - custom `#[derive]` used on structs and enums
///     - Attribute-like macros that define custom attributes usable on any item
///     - Function-like macros that look like function calls but operate on the tokens specified as their argument
/// 
/// ====== Difference between macros and functions
//////////////////////////////////////////////////
/// Fundamentally, macros are a way of writing code that writes other code
/// which is known as `metaprogramming`, its useful to make writing code shorter.
/// Macros can take a variable amount of arguments.
/// Macros are expanded before the compiler interprets the meaning of the code,
/// as a result a Macro can implement a trait on a specific type unlike a function.
/// Macros: you write Rust code that writes other Rust code.
/// Functions can be defined anywhere and called anywhere. Macros need to be brought into scope before calling.
/// 
/// ====== Declarative Macros with `macro_rules!` for General Metaprogramming
/////////////////////////////////////////////////////////////////////////////
/// At their core they allow you to write something similar to a `match` expression
/// Macros compare values with patterns that are associated with particular code (the return value of 
/// the match expression is the code it self)
/// To define a macro you use the `macro_rules`` construct.
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ), * ) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }
}
fn main() {
    println!("Hello, world!");
}
