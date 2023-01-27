fn main() {
    // ======== Slice type
    // It fixes one common problem in programming which is:
    // Values that are calculated from data in a particular state but aren't tied 
    // to that state at all. So when the state updates the Values don't, and thus they 
    // become invalid.

    // Let's say we want to get the first word in this text
    let text = String::from("first second");
    // Without slices we would have to do it like this
    let word1 = first_word(&text); // word will get the value 6
    println!("first index {word1}");
    // text.clear(); // text is empty
    // first_word now has a value completely independent from text
    // to solve this we'll use Slice
    let word2 = first_word_fix(&text);
    println!("first word fix {word2}");
    // the compiler will now make sure this code is valid
    // we can't call
    // text.clear(); // this will throw an error
    // because word2 is a reference to text which is a String
    // This is because one of the borrowing rules is that if we have an immutable reference to a
    // value we can't have a mutable reference to it as well.

    // String literals 
    let string_literal = "hello world";
    // are string slices already.
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_fix(s: &str) -> &str { // str is a string slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}