mod rectangle;
mod how_tests_are_run;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}

pub fn should_panic() {
    panic!("panic error");
}


#[cfg(test)]
mod tests {
    use super::*;
    
    // Test function in rust are functions annotated with the 'test' attribute
    // Attributes are metadata about pieces of Rusts code
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        // we also have assert_ne!
    }

    #[test]
    fn greeting_works() {
        let result = greeting("hamza");
        // we can add a message to the assert macros
        assert!(
            result.contains("hamza"),
            "Greeting did not contain {}",
            result
        );
    }

    // we can check for panic by using should_panic
    #[test]
    #[should_panic]  
    fn should_panic_works() {
        should_panic();
    }

    // to make our should_panic tests more precise, we can add an optional `expected` parameter to the `should_panic`
    // attribute
    #[test]
    #[should_panic(expected = "panic error")]
    fn should_panic_with_extra_details() {
        should_panic();
    }

    // we can also use Result<T, E> 
    // This is useful as it enables us to write tests that use the ? operator inside the tests
    #[test]
    fn using_result() -> Result<(), String> {
        let result = add(3, 5);
        if result == 8 {
            Ok(())
        } else {
            Err(String::from("An error happened"))
        }
    }

}
