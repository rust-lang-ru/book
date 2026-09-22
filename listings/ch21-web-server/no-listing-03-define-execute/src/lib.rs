pub struct ThreadPool;

// ANCHOR: here
impl ThreadPool {
    // --snip--
    // ANCHOR_END: here
    pub fn new(размер: usize) -> ThreadPool {
        ThreadPool
    }

    // ANCHOR: here
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
// ANCHOR_END: here
