use std::collections::HashMap;

pub fn testing_hash_maps() {
    // Storing keys with associated values in Hash Maps `HashMap<K,V>`
    // The data is also stored on the heap
    // HashMaps are homogeneous, all keys must have the same type as each other, and all the values must also have the same type
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Adding a Key and Value only if a key isn't present
    // we have the `entry` method
    scores.entry(String::from("yellow")).or_insert(40);

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert returns a mutable reference
        let count = map.entry(word).or_insert(0);
        // * is a dereference operaor
        *count += 1;
    }
}
