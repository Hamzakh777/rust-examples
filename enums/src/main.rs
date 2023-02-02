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