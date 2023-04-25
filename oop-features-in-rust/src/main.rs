mod trait_objects;
mod oop_pattern;

fn main() {
    println!("Hello, world!");
    trait_objects::run();
    trait_objects::drive_reference_dyn();
    oop_pattern::blog::run();
}
