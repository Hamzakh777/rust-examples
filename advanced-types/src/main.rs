// ===== Newtype pattern for safety and abstraction
///////////////////////////////////////////////////
// The program won't compile if we try to use a NewType
// inplace of the type it wraps or vice-versa
// struct Meter(u32)
// fn test(a: u32);
// test(Meter(2)) WONT WORK

// We use the Newtype pattern to abstract away some implementation details
// the new type can expose a public API that is different from the aPI of the private 
// inner type.
// We also use NewType to indicate the units of values, for example instead of using 
// u32 we'd use struct Meters(u32) to indicate what the integer corresponds to.

// ===== Type Aliases
/////////////////////
// They allow you to write shorter code when you have long types.
// This is different from `NewType` pattern as this doesn't create a new type
// but just an alias, 
type Length = u32;
fn test_type_alias() {
    let a: Length = 2;
    let b: u32 = 3;
    println!("{}", a + b); // this works with Type Alias, but won't work with NewType
}
type Thunk = Box<dyn Fn() + Send + 'static>

// ====== The Never Type that Never Returns
///////////////////////////////////////////
fn bar() -> ! {
    // -> snip
}
// ! is the Never Type
// ! can be coerced into any other type
// this is useful when working with match and each match arm need to return the same type
// let guess: u32 = match guess.trim().parse() {
//   Ok(num) => num,
//   Err(_) => continue,
// };
// Rust looks at num which has a type of u32 and continue which has a type of !, then gives guess type u32
// `continue` has the ! type
// panic! also has the ! type.

// ====== Dynamically Sized Type and the `Sized` Trait
//////////////////////////////////////////////////////
/// Rust needs to know certain details about its types, such as how much space to allocate for a value 
/// of a particular type.
/// `dynamitcally sized types` or DSTs or `unsized types` these types allow us to write code
/// using values whose size can only be known at runtime.
/// `str` is a dynamically sized type
/// This code will fail, because each variable will require a different memory size and they both use the same type `str`
/// let s1: str = "asdf"; 
/// let s2: str = "asdf 23";
/// `str` is different from `&str`
/// The way to go around this is by using `&str` which stores the starting position and the length of the data.
/// The way most dynamically sized types work in rust is by adding some metadata to the pointer
/// We can combine `str` with all kinds of pointers Box<str>, Rc<str>.
/// traits are also dynamically sized types (Ex: when using trait objects &dyn TraitName)
/// To work with DSTs, Rust provides `Sized` trait to determine whether or not a type's size is known at compile time
/// This trait is automatically implemented for all the types whose size is known at compile time.
/// By default, generic functions will only work on types that have a known size at compile time.
/// fn generic<T: Sized>(t: T) {}
/// You can use this syntax to relax this this restriction; 
/// fn generic<T: ?Sized>(t: &T) {}
/// Also note that we switched the type of the t parameter from T to &T. Because the type might not be Sized, 
/// we need to use it behind some kind of pointer. In this case, we’ve chosen a reference.

fn main() {
    println!("Hello, world!");
}
