extern crate trpl; // требуется для mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // We will call `slow` here later
    });
}

// ANCHOR: slow
fn медленно(имя: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{имя}' выполнено за {ms}ms");
}
// ANCHOR_END: slow
