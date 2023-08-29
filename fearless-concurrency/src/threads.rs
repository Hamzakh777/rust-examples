// Each thread had its own Stack, Registers and Program Counters
// A process has multiple threads
// Each process has its own memory and address space, one process cannot corrupt other process's memory,
// this means when one process malfunctions other processes keep running
// The threads do share the same memory under the Process

// Rust standard library uses 1:1 model of thread implementation, whereby a program uses one operating system
// thread per one language thread.

// When the `main` thread of a Rust program complete, all the spawned threads are shut down
// wether or not they have finished running.

// We can wait for All threads to Finish using the `join` handles

use std::thread;
use std::time::Duration;

fn thread_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // this forces the thread to stop its execution for a short duration allowing a different thread to run.
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if we move `handle.join()` here we would get a different result
    // where the spawned thread has to finish executing before the rest of the code is executed

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates.
    // Blocking = prevented form performing work or exiting.
    handle.join().unwrap();
}

fn move_closures_with_threads() {
    // variables declared in the main thread can't be accessed inside the spawned thread unless we use
    // `move` keyword. 
    // The reason for that is that the variables might be dropped on the main thread before the spawned thread
    // finishes execution, and we'll have an invalid pointer.
    let v = vec![1, 2, 3];

    // `move` forces the closure to take ownership of the values its using rather than allowing Rust
    // to infer that it should borrow the values.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn run() {
    thread_example();
    move_closures_with_threads();
}
