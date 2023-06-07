/// ====== Function pointers
////////////////////////////
/// You can pass regular functions to functions
fn add_one(num: u32) -> u32 {
    num + 1
}  
/// Here we are passing add_one as an argument
fn do_twice(f: fn(u32) -> u32, num: u32) -> u32 {
    f(num) + f(num)
}
/// Unlike closures (Fn trait), we don't need generics, so we specify `fn` directly as the parameter type.
/// Function pointers implement all three of the closures traits (Fn, FnMut and FnOnce) meaning
/// YOU CAN ALWAYS PASS A FUNCTION POINTER TO A FUNCTION THAT EXPECTS A CLOSURE.
/// Unlike closures (Fn trait), `fn` is a type rather than a trait.
/// Each enum variant that we define also becomes an initializer function.
enum Status {
    Value(i32),
    Stop,
}
fn enum_variant_initializer() {
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
/// Both closures and function pointers compile down to the same code, so use whichever you like

/// ===== Returning Closures
////////////////////////////
/// Closures are represented by traits, which means you cant return closures directly.
/// In most cases where you want to return a trait you can return the concrete type 
/// that implements that trait. But you can't do that with closures because they don't have 
/// a concrete type.  
/// Because Rust doesn't know how much space it will need to store the closure. We have 
/// use smart pointers
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    println!("Hello, world!");
}
