pub fn demo() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

struct Point<T, U> {
    x: T,
    y: U,
}

// we have to declare `T` just after `impl` so we can use `T` to specify that we're
// implementing methods on the type `Point<T>`.
// So Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type.
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    // the method can also have its own different generic types
    fn mixup<X, Z>(self, other: Point<X, Z>) -> Point<T, Z> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point2<T> {
    x: T,
    y: T
}

// we can implement methods on `Point<f32>` instances rather than on `Point<T>` instances
impl Point2<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

