use core::fmt;
use std::ops::Add;

// ===== Associated Types
// They are like defining an alias inside the Trait itself
trait Iterator {
    type Item; // this is an associated type

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter;
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(2)
    }
}
// they act similar to generics but the main different between the two is that with
// Generics you can implement the same multiple times with different generic type parameters.
// so when you want to use the Type that implements the traits you have to annotate it so Rust
// knows which trait implementation to use.
// As for associated types they can only be implemented once.

// ===== Default Generic Type Parameters
// this is essentially providing a default type for the Generic type parameter
trait Add2<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

// ===== Operator overloading
// Rust has traits that you can implement on custom types to add support for certain operators like `+`.
// You can overload the operations and corresponding traits listed in `std::ops`
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// another more complex example
#[derive(PartialEq, Debug, Clone, Copy)]
struct Millimeters(u32);
#[derive(PartialEq, Debug, Clone, Copy)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    // rhs stands for right-hand-side
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// Use cases for `default type parameters`
// 1. to extend a type without breaking existing code (Ex: if you want to add a type parameter to an existing trait)
// 2. to allow customization in specific cases most users wont need (Like the Add trait)

fn test_operator_overloading() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 1, y: 0 },
        Point { x: 2, y: 0 }
    );

    assert_eq!(Millimeters(1) + Meters(3), Millimeters(3001));
}

// ===== Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
trait Pilot {
    fn fly(&self);
}
trait Wizzard {
    fn fly(&self);
}

struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizzard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("waving arms furiously!");
    }
}

// by default when calling a method defined in the type and the traits that it implements,
// the compiler will call the method that is directly implemented on the type
fn test_disambiguation() {
    let person = Human;
    person.fly(); // "waving arms furiously"

    // if we want to call a specific method
    Pilot::fly(&person);
    Wizzard::fly(&person);

    // we can even do
    Human::fly(&person); // equivalent to person.fly()

    // Specifying the trait name before the method name clearly tells Rust
    // which method to call
}
// associated functions are methods that don't take 'self' as their first parameters.
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
fn example_calling_associated_function() {
    // calling Dog::baby_name() will call the associated function defined on the struct itself
    // to tell Rust to call instead the method on the trait implementation we do the following.
    // Its very similar to type casting, but in Rust its called `Fully qualified syntax`
    println!("{}", <Dog as Animal>::baby_name());
}
// Generally, fully qualified syntax looks as follows
// `<Type as Trait>::function(receiver_if_method, next_arg,...);`
// associated functions that are not methods
// You only need to use this verbose syntax in cases where the Rust compiler can't figure out
// on its own which function to call

// ====== Using Supertraits to require one trait's functionality within another trait
/////////////////////////////////////////////////////////////////////////////////////
// Sometimes you might require that if the user wants to implement a Trait he has
// to implement another Trait
// The trait that my trait is relying on is called 'Supertrait', so you can use the associated
// items defined in this Supertrait
// trait MyOwnTrait: TraitThatINeed {
//     fn test() {
//      }
// }
// This is essentially saying: OutlinePrint requires implement the Display trait as well
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// ====== Using the NewType Pattern to implement external Traits on external types
////////////////////////////////////////////////////////////////////////////////
// In chapter 10 we mentioned Rust orphan rule, where external traits can't be implemented on 
// external types. This is to prevent randomly adding/changing behavior for 
// types in the standard library or an external crate we are using.

// we can create a wrapper around other types to overcome the orphan rule
// to create a wrapper we use a Struct Tupe with one field
struct Wrapper(Vec<String>);
// and then instead of using Vec<String> we just use our type Wrapper
// the orphan rule prevents us from implementing external Traits on external Types
// so we can't do 
// impl fmt::Display for Vec<String>
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
fn using_the_newtype() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
// there is no performance cost on this technique
// The downside of this technique is that Wrapper doesn't have the methods of the value
// it's holding. We would have to manually implement all the methods of `Vec<T>` directly on 
// `Wrapper` such that the methods delegate to `self.0` which would allow us to treat 
// Wrapper exactly like Vec<T>. Implementing the `Deref` trait on the Wrapper will allow us 
// to have every method the inner type has
fn main() {
    println!("Hello, world!");
}
