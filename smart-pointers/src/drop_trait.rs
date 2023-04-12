// The `Drop` trait lets you customize what happens when a value goes out of scope.
// The functionality of the `Drop` trait is almost always used when implementing a smart pointer.

struct CustomSmartPointer {
    data: String,
}

// The Drop trait is included in the prelude, we don't need to bring it into scope
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("droping the value {}", self.data);
    }
}

pub fn run() {
    let pointer_1 = CustomSmartPointer {
        data: String::from("pointer 1")
    };

    let pointer_2 = CustomSmartPointer {
        data: String::from("pointer 2")
    };

    println!("Run function finished executing");
}

// variables are dropped in reverse order

// ===== Dropping a value early with `std::mem::drop`
// Calling `drop` manually is not allowed and the compiler won't allow it.
// Occasionally, we might want to drop a value early. 
// One example is when using smart pointers to manage locks
// https://doc.rust-lang.org/stable/book/ch15-03-drop.html#dropping-a-value-early-with-stdmemdrop


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }
}