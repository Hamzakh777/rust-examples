// A trait can define functionality a particular type has and can share with other types.
// Traits are similar to interfaces in other languages, although with some differences

// The compiler will enforce that any type that has the `Summary` trait will have the method
// `summarize` defined with this signature exactly

// The user will have to bring the Trait into Scope in order to be able to use it as well as the Types

// We can't implement external Traits on external Types. Ex: We can't implement the `Display` trait on `Vec<T>`
// This restriction is part of a property called `coherence`, more specifically `orphan` rule. This ensures
// that other people's code can't break my code and vice versa.

// Traits can also have default behaviors for some or all the methods in a trait

// Default implementations on a trait can call other methods in the same trait, even if those other methods don't
// have a default implementation.

use std::fmt::{Debug, Display};

// Media aggregator examples
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String;

    fn title(&self) -> String {
        format!(
            "title should be returned from this function {}",
            self.summarize_author()
        )
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// ====== Traits and parameters
// We can use traits to define functions
// that accept many different types as long as they implement that trait

// This is syntactic sugar for something called `trait bouding syntax`
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// `trait bound syntax`
pub fn notif2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// we can force multiple parameters to have the same type
pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}
// we can have multiple `trait bounds` with the `+` sign
// multiple trait bounds means that the type should implement multiple traits
// pub fn notify4(item: &(impl Summary + Display))
pub fn notify4<T: Summary + Display>(item: &T) {}
// we can have clearer Trait bounds by using the where clause
// this is useful in case we have multiple generic type parameters
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Debug + Clone,
{
    4
}
// we can also define return types using Traits
// meaning the return value should a Type that implements that particular Trait
// This is especially useful in the context of CLOSURES AND ITERATORS
// This only works if your function returns just one type.
// if the function can return different types that implement the Summary Trait it won't compile
pub fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// ==== Blanket implementations
// https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
// Implement a trait for any Type that implements another trait
// Conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

// This will be only implemented for trait bounds that satisfy the requirement
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn demo() {}
