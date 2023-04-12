mod box_t;
mod deref_trait;
mod drop_trait;
mod rc_smart_pointer;
mod refcell_t;
mod memory_leak;

fn main() {
    println!("Hello, world!");

    memory_leak::run();
}
