#[derive(Debug)]
pub struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod RectangleTest {
    use super::*;

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            height: 10,
            width: 10,
        };

        let smaller = Rectangle {
            height: 5,
            width: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }
}