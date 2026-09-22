pub struct ThreadPool;

// ANCHOR: here
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the число of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(размер: usize) -> ThreadPool {
        assert!(размер > 0);

        ThreadPool
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
