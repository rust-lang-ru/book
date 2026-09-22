// ANCHOR: here
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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

        let mut threads = Vec::with_capacity(размер);

        for _ in 0..size {
            // create some threads and склад them in the vector
        }

        ThreadPool { threads }
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
// ANCHOR_END: here
