pub fn testing_vectors() {
    let v: Vec<i32> = Vec::new();
    // we can also use the vec! macro to initiate a vector
    let v2 = vec![1, 4, 5, 6, 7]; // this will have the type of Vec<i32> as i32 is the default integer type
                                  // We can add items to a vector - Notice how Rust inferes the type after calling .push()
    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(3);

    // reading elements from a vector can be done in two way
    // using indexing and using `get`
    let first_value: &i32 = &v2[0]; // this is a reference because its an expression
    let last_value: Option<&i32> = v2.get(4);
    match last_value {
        Some(&value) => println!("the last value is {value}"),
        None => println!("Nothing"),
    }

    // we can also loop over the vector by using for
    // we also have to pass a reference to the vector here
    for i in &v2 {
        println!("the number is {i}");
    }
    // we can also use the for loop to update the items in the vector
    let mut mutable_vec = vec![2, 3, 4, 5, 6, 6, 4, 2, 1];
    // if we want to update the vector
    for i in &mut mutable_vec {
        // * is used for called the dereference operator, its essentially following the pointer to the value
        *i += 50;
        println!("the new value is {i}");
    }

    // Vectors can only store values of the same type, we can use an enum to go around this 
    // because all the variants of an enum are defined under the same enum type
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Float(34.44),
        SpreadsheetCell::Text(String::from("value")),

    ];
    // using an enum plus a match expression will enable rust to make sure every case is handled
}
