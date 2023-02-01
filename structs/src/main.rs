struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// this is called an attribute
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    println!("Hello, world!");
    // structs are similar to Tuples, in that they both hold multiple related values.
    // This is pretty much like defining a type/interface in Typescript
    // to use a Struct, we have to define an instance like follows
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // An instance can be mutable
    // let mut user1 = User...
    // The whole instance needs to be mutable, Rust doesn't allow us to only mark certain fields as mutable

    // Struct update syntax, like destructuring in js
    let user2 = User {
        email: String::from("email@test.com"),
        ..user1
    };
    // The Struct update syntax uses = like an assignment, 
    // this is because it moves the data, so user1 is no longer valid 

    // ======= Tuple Structs
    // Rust also supports structs that look similar to tuples, called `tuple structs`
    // its adding a name to a tuple
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // Its useful when you want to give a Tuple a name or make it different from other Tuples
    // that have the same structure.
    // Types are nominal instead of structural like in Typescript

    // ======== Unit-like structs
    // we can define Structs that don't have any fields, these are called `unit-like structs`
    // because they behave similar to () 
    // Unit like structs are useful when you want to implement a trait on some type but don't have 
    // any data that you want to store in the type itself
    struct AlwaysEqual;

    // ======= Ownership of Struct data
    // We use String when defining the User Struct because we want it to own
    // all of its data and for that data to be valid as long as the entire struct is valid.
    // its also possible for structs to store data owned by something else (a reference), but 
    // that requires the use of life-times.

    // :? inside the println! macro tells the compiler to call Debug instead of the Display trait
    let rectangle = Rectangle {
        height: 10,
        width: 20,
    };
    println!("rectangle is {:?}", rectangle); // annotating `Rectangle` with `#[derive(Debug)]` fixes this issue
    // we can also use {:#?} to print the Struct in a much nicer output
    println!("the width is {}", calculate_area(&rectangle));

    // we can also use dbg! macro to debug structs
    // but unlike the println! macro, it takes ownership of the variable
    let scale = 2;
    let rectangle = Rectangle {
        height: 50,
        width: dbg!(30 * scale) // we can also do this since dbg! returns ownership of the expressions value.
    };
    dbg!(&rectangle); // we can also pass a reference
    // dbg! outputs to stdeer unlike println! which out puts to stdout
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // we can use shorthand like javascript
        email,
        sign_in_count: 1,
    }
}