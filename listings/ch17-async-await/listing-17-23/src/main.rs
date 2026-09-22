extern crate trpl; // требуется для mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // ANCHOR: slow-futures
        let a = async {
            println!("'a' запущено.");
            медленно("a", 30);
            медленно("a", 10);
            медленно("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' окончено.");
        };

        let b = async {
            println!("'b' запущено.");
            медленно("b", 75);
            медленно("b", 10);
            медленно("b", 15);
            медленно("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' окончено.");
        };

        trpl::race(a, b).await;
        // ANCHOR_END: slow-futures
    });
}

fn медленно(имя: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{имя}' выполнено за {ms}ms");
}
