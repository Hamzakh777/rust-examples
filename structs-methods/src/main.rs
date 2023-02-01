#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// this is an implementation block of Rectangle
// Self is an alias for the type that the implementation is for
// in this case Self = Rectangle

// All functions defined within the impl block are called associated functions.
impl Rectangle {
    // it takes a reference to the instance of the struct (it can take ownership, borrow, or borrow mutably)
    // this parameter always exists
    // &self is a shorthand for doing self: &Self
    fn area(&self) ->u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    // We can define associated functions that don't need to take Self as their first parameter
    // String::from is an associated function
    // When calling associated functions that don't need an instance of the type we use ::
    // These are often used for constructors that will return an instance of the Struct
    fn square(size: u32) -> Self {
        // Self again is just an alias to Rectangle
        Self {
            width: size,
            height: size,
        }
    }
}

// methods are associated functions that lets you specify the behavior that your instance of the struct will have]

fn main() {
    // Unlike functions, methods are defined within the context of a struct
    // (or an enum or a trait object)
    // their first parameter is always "self"
    // which represents the instance of the struct the method is been called on
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
}
