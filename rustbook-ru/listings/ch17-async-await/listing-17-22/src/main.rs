extern crate trpl; // required for mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // We will call `slow` here later
    });
}

// ANCHOR: slow
fn slow(имя: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{имя}' ran for {ms}ms");
}
// ANCHOR_END: slow
