// We will be a GUi that calls a method draw on an object
// In oop we would use inheritance by defining a parent class
// named `Component` that has the draw function implemented. 
// And then the sub-classes (Input, Image) will inherit it.

// In rust we have to use `Trait objects` to achieve a similar behavior
// Then we can define a Vector that takes a `Trait object`.
// A `Trait object` points to both an instance of a type implementing our specified trait
// and a table used to look up trait methods on that type at runtime.
// To make it, we use some sort of pointer `&` or a `Box<T>`, then the `dyn` keyword, and then the relavant trait

// We have to use Box/& because we don't know the size of the type at compile time

// ==== Trait objects allows abstraction across common behavior (polymorphism)

use self::{select_box::SelectBox, button::Button};

mod button;
mod select_box;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>> // This is a trait object, any type inside a box that implements the Draw trait
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub fn run() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// Traits objects use `dynamic dispatch` unlike `Trait bounds` which use `static dispatch`
// - dynamic dispatch: determining which implementation of a method or a function to call at runtime
// - static dispatch: determining which implementation of a method or a function to call at compile-time
// This means there is some extra cost to incur at run-time.

// `dyn`: Can be a reference or a smart pointer (Box<T>)

// `dynamic disptach`: uses a fat-pointer, where one pointer points to the data and the other on a lookup table
// for which function to execute.

// Another example

trait RoadLegal {
    fn driveOnRoad(&self);
}

struct Car;
impl RoadLegal for Car {
    fn driveOnRoad(&self) {
        println!("driving on road the car");
    }
}

struct ATV;
impl RoadLegal for ATV {
    fn driveOnRoad(&self) {
        println!("driving on road the atv");
    }
}

fn drive_a_road_legal_vehicule(vehicule: &dyn RoadLegal) {
    vehicule.driveOnRoad();
}

pub fn drive_reference_dyn() {
    let car = Car {};
    let atv = ATV {};
    drive_a_road_legal_vehicule(&car);
    drive_a_road_legal_vehicule(&atv);
}