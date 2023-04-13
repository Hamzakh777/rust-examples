// *message passing* is one increasingly popular approach to ensuring safe concurrency.
// where threads communicates with each other by sending messages using channels instead
// of sharing memory to communicate.
// The ownership rules plays a vital role in message sending because they help you write safe and current code.

use std::{sync::mpsc, thread, time::Duration}; // mpsc stands for multiple producer, single consumer;

fn channels_example() {
    let (tx, rx) = mpsc::channel();

    // the spawned thread needs to own the transmitter in order to send
    // messages through the channel.
    thread::spawn(move || {
        let val = String::from("hi");
        // `send` takes ownership, so we can't use this value again inside this thread.
        tx.send(val).unwrap();
    });

    // `receiver` has two useful methods: `recv` and `try_recv`
    // `recv` blocks the main thread execution and wait until a value is send down the channel.
    // `try_recv` doesn't block. It immediately returns Result<T, E>. This is useful if the main
    // thread has other work to do. We could put it inside a loop that calls `try_recv`
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn send_multiple_values_and_seeing_the_receiver_waiting() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    for received in rx {
        println!("received {}", received);
    }
}

fn creating_multiple_producers_by_cloning_the_transmitter() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("1 from thread 1"),
            String::from("2 from thread 1"),
            String::from("3 from thread 1"),
            String::from("4 from thread 1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("1 from thread 2"),
            String::from("2 from thread 2"),
            String::from("3 from thread 2"),
            String::from("4 from thread 2"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    for received in rx {
        println!("received {}", received);
    }
}

pub fn run() {
    channels_example();
    send_multiple_values_and_seeing_the_receiver_waiting();
    creating_multiple_producers_by_cloning_the_transmitter();
}
