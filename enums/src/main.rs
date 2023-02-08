enum IpAddrKind {
    V4,
    V6,
}

// we can put data directly under each enum variant
// this is useful incase we want to attach data with the variant, we can use
// a struct + enum to achieve something similar, but that's too messy.
enum IpAddr {
    V4(String),
    // the name of each variant we define, 
    // also because a function that constructs an instance of the enum.
    // That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.
    // you can put any kind of data inside the enum variant, simple types, structs, tuples,... even another enum
    V6(String),
}

// ======= null
// Rust doesn't have the null value. 
// But it does have the Option enum to express a similar concept.
// The Option<T> enum is included in the prelude (the stuff available in the global scope without having to explicitly import it)
// its variants (Some, None) are also included in the prelude
// using Option explicitly forces us to handle the case where the value can be None. 
// That's why Option<T> is different than T
// This helps catch one of the most common issues which is assuming that something isn't null when it actually is
// the compiler will make sure we handle the null (none) case before we try to use the value
//  In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. Then, when you use that value, you are required to explicitly handle the case when the value is null. 
// more info here https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}