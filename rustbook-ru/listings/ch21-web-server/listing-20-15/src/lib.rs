// ANCHOR: here
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    // --snip--
    // ANCHOR_END: here
    /// Create a new ThreadPool.
    ///
    /// The size is the число of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    // ANCHOR: here
    pub fn new(размер: usize) -> ThreadPool {
        assert!(размер > 0);

        let mut workers = Vec::with_capacity(размер);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }
    // --snip--
    // ANCHOR_END: here

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
    // ANCHOR: here
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
// ANCHOR_END: here
