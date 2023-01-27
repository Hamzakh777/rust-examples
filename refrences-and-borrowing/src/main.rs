fn main() {
    // A reference is like a pointer that we can follow to access the data stored at that address
    // we use & to tell the function that it expects a reference 
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The lenght of the string is {len}");
    // We call the action of creating references borrowing
    // refrences by default are immutable

    // ====== Mutable references
    let mut s = String::from("hello");
    // the function signature need to accept a mutable reference
    change(&mut s);
    println!("{s}");

    // To prevent data races, Rust lets you borrow at max one mutable reference
    // If you have a mutable reference to a value, you can have no other references to that value

    // we can't have an immutable and mutable references to the same value.
    // Because users of an immutable variable don't expect it to suddenly change out from under them.
    // multiple immutable references are allowed, because no one who is is just reading the data has the ability to effect
    // anyone else's reading the data.

    // ======== Dangling references
    // Rust guarantees that refrences will never be dangling references
    dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", worlds");
}

fn dangle() -> String {
    // S gets created
    let s = String::from("string");
    // &s // this code will return an error
    // The fix is to simply return the value;
    s
} // s goes out of scope so its value is dropped, thus there is nothing to reference
