use std::{sync::{mpsc::{self, Receiver}, Arc, Mutex}, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        // with_capacity is similar to `Vec::new` but it preallocates space in the vector
        // doing it upfront is slightly more efficient than using `Vec::new` which resizes itself as elements are added
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        // We have to use Arc (thread-safe smart pointer) to share ownership across multiple threads
        // and allow threads to mutate the value we need to use Arc<Mutex<T>>.
        // Arc will allow multiple Wokers to use the receiver
        // while Mutex will ensure only one worker gets a job from the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        // create some threads and store them in the vector
        // we want to create threads and have them `wait` for code to that we will send later
        for id in 0..size {
            // we clone Arc to bump the reference count.
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F) 
    where 
        // : we need Send to transfer the closure from one thread to another 
        // and 'static because we don’t know how long the thread will take to execute.
        // We still use the () after FnOnce because this FnOnce represents 
        // a closure that takes no parameters and returns the unit type ()
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

// we want Worker to fetch the code to run from a queue under ThreadPool
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        // thread::spawn will panic if the system fails to create a thread (because of resource limit for example)
        // this will cause our program to crash 
        // in production, use `std::thread::Builder` which returns a `Result` instead of panicking
        // We have to loop instead of while because of:
        // the Mutex struct has no public unlock method because the ownership of the lock is based on the lifetime of the MutexGuard<T> within the LockResult<MutexGuard<T>> that the lock method returns. At compile time, the borrow checker can then enforce the rule that a resource guarded by a Mutex cannot be accessed unless we hold the lock. However, this implementation can also result in the lock being held longer than intended if we aren’t mindful of the lifetime of the MutexGuard<T>.
        // more here: https://doc.rust-lang.org/book/ch20-02-multithreaded.html#implementing-the-execute-method
        // in short: `let` drops any temporary values by the end of expression. 
        // `if let`, `while let` and `match` does not drop temporary values until the end of the associated block.
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job");

            job();
        });

        Self {
            id,
            thread
        }
    }
}