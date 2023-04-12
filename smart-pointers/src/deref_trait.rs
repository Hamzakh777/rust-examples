use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for MyBox<T> {
    // associated type, they are a slightly different way of declaring a generic parameter
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// calling *y on MyBox is equivalent to *(y.deref()), but rust takes of this stuff under the hood
// the reason of having to still use * outside (y.deref()) is becuase of the ownership, 
// if the `deref` returned the value directly instead of a reference to the value, the value would be moved
// out of `self`.

// Without the Deref trait the compiler can only dereference `&` references

// ====== Implicit Deref Coercions with Functions and Methods
// `Deref coercion` converts a reference to a type that implements the `Deref` trait into a reference to another type

#[cfg(test)]
mod tests {
    
    // Box<T> implements the `Deref` trait which allows us to dereference the pointer
    // - dereferencing is following the pointer to the value it points to
    // Example
    #[test]
    fn deref_example() {
        let x = 5;
        let y = &x; // this is a reference to x

        assert_eq!(x, *y);
    }
}