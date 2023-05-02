// ==== Unsafe Rust
// It doesn't enforce memory safe guarantees
// Another reason is that the underlying computer hardware is inherently unsafe
// Certain tasks requires `unsafe` code

// You can take five action in `unsafe` Rust that you can't take in normal Rust
// 1. Dereference a raw pointer
// 2. Call an unsafe function or method
// 3. Access or modify a mutable static variable
// 4. Implement an unsafe trait
// 5. Access fields of `union`S

// `unsafe` doesn't turn off the borrow checker or Rusts other safety features

// Keep `unsafe` blocks small

// It's best to enclose `unsafe` code with a safe abstraction and provide a safe API, parts of the standard 
// library are implemented this way.

pub fn run() {

}

// 1.
fn dereferencing_a_raw_pointer() {
    // We get `raw pointers` inside `unsafe` rust bocks
    // Raw pointers can be mutable or immutable and are written like '*const T' or '*mut T'
    // the * isn't a dereferencing operator, its part of the type name

    let mut num= 5;

    // we can create an immutable pointer and mutable pointer to the same location
    // and change the data through the mutable pointer (this can create a data race).
    // Real world use case: interfacing with C code. Or building safe abstractions 
    // that the borrow checker doesn't understand. 
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // notice we didn't use `unsafe`, that's because we can create `raw pointers in 
    // safe code, we just can't dereference them (read the data being pointed to)

    // We can read any random memory address
    let address = 0x12345usize;
    let r = address as *const i32;
}

// 2.
// Unsafe functions and methods look exactly like normal functions and methods,
// but they have an extra unsafe before the rest of the definition.
// Unsafe functions needs to be called inside unsafe blocks
unsafe fn calling_an_unsafe_function_or_method() {
// we don't need to add `unsafe` blocks inside `unsafe functions`    
}

// split_at_mut is a common function that safe but implement unsafe code inside
// this method is defined on mutable slices.
fn split_at_mut_example() {
    let mut v = vec![1,2,3,4,5,6];

    let r = &mut v[..];

    // we can't implement `split_at_mut` using safe rust
    // because the compiler wont let us have to mutable references to the 
    // same variable. Even thought both of these references point to indapandant data.
    // we are borrowing different parts of the slice.
    // slices are a pointer to some data and the length of that data
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);
}

fn using_extern_functions_to_call_external_code() {
    // sometimes your code might need to work with other languages 
    // use of a Foreign Function Interface (FFI) is facilitated by the `extern` keyword.
    // FFI is a way for programming languages to define functions and enable a different (foreign)
    // programming language to call those functions.

    // the 'C' part defines with ABI the external function uses
    // 'C' is the most common and follows the C programming language's AbI.
    extern "C" {
        // we list the function signature of external functions we want to call
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // the usage of `extern` doesn't require `unsafe`
}
// we can also specify rust functions that can be called from other languages
// no_mangle is to not change the function signature at compile time - not to mangle
// the name of the function
#[no_mangle] 
pub extern "C" fn call_from_c() {
    println!("Just a rust function from C!");
}

//.3
// global variables are called static variables
// static variables are similar to constants
// their lifetime needs to be 'static
// Rust supports global variables but can be problematic because of the ownership rules
// If two threads are accessing the same mutable global variable it can cause a data race.
static HELLO_WORLD: &str = "Hello world"; 
static mut COUNTER: u32 = 0;
fn accessing_or_modifying_a_mutable_static_variable() {
    println!("name is: {}", HELLO_WORLD);

    // The different between `constants` and `immutable static variables` is that values 
    // in a static variable have a fixed address in memory.
    // Constants are allowed to duplicate their data whenever they're used
    // Accessing and modifying static variables is `unsafe`

    unsafe {
        COUNTER += 1;
    }

    // Having multiple threads access `COUNTER` would likely result in a data race.
}

//.4 - unsafe traits
// When at least one of its methods has some invariant that the compiler can't verify.
unsafe trait Foo {}

unsafe impl Foo for i32 {}
// example for using this is with `Send` and `Sync` traits
// if a raw pointer is used in our type, if we want to mark it as `Send` or `Sync` we must use `unsafe`
// Rust can't verify that our type can be safely sent between threads.

//5. Accessing fields of a union
// unions are similar to structs, but only one declared field is used in a particular instance at one time.
// there are mainly used to interface with unions in C code