// ANCHOR: here
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
// ANCHOR_END: here
