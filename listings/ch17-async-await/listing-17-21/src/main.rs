extern crate trpl; // требуется для mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: here
        let slow = async {
            println!("'slow' запущено.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' окончено.");
        };

        let fast = async {
            println!("'fast' запущено.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' окончено.");
        };

        trpl::race(slow, fast).await;
        // ANCHOR_END: here
    });
}
