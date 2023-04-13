// Mutex is a way to communicate between threads by sharing data
// Mutex stands for mutual exclusion
// A mutex works by having a lock, where only one thread can access the data at a time. 
// It has an internal data structure to track who is using the lock
// When done with the data you must unlock the lock so other threads can use the data.
// Thanks to Rust type system you can't get locking and unlocking wrong

// Mutex provides interior mutability as the `Cell` family does (RefCell)

// Mutex comes with the risk of creating DeadLocks. These occur when an operation needs 
// to lock two resources and two threads have acquired one of these locks, causing them
// to wait for each other forever

use std::{sync::{Mutex, Arc}, thread, time::Duration};

fn mutex_simple_example() {
    // Mutex<T> is a smart pointer
    let m = Mutex::new(5);

    {
        // we have first to acquire the lock
        // MutexGuard is a smart pointer has Drop implementation where it releases the lock automatically
        let mut m = m.lock().unwrap();

        *m = 6;
    }

    println!("M's value {:?}", m);
}

fn sharing_mutex_between_multiple_threds() {
    // atomic reference counting with Arc
    // `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations
    let counter = Arc::new(Mutex::new(0)); // `counter` is immutable but we could get a immutable reference to the value inside of it
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        // we call `join` to make sure all threads finishes
        handle.join().unwrap();
    }

    println!("Counter {:?}", counter);
}

// fn deadlock_example() {
//     let data = Arc::new(Mutex::new(0));
//     let d1 = data.lock();
//     let d2 = data.lock(); // cannot lock, since d1 is still active
// }

pub fn run() {
    mutex_simple_example();
    sharing_mutex_between_multiple_threds();
    // deadlock_example();
}