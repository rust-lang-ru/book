extern crate trpl; // требуется для mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // ANCHOR: yields
        let a = async {
            println!("'a' запущено.");
            медленно("a", 30);
            trpl::yield_now().await;
            медленно("a", 10);
            trpl::yield_now().await;
            медленно("a", 20);
            trpl::yield_now().await;
            println!("'a' окончено.");
        };

        let b = async {
            println!("'b' запущено.");
            медленно("b", 75);
            trpl::yield_now().await;
            медленно("b", 10);
            trpl::yield_now().await;
            медленно("b", 15);
            trpl::yield_now().await;
            медленно("b", 350);
            trpl::yield_now().await;
            println!("'b' окончено.");
        };
        // ANCHOR_END: yields

        trpl::race(a, b).await;
    });
}

fn медленно(имя: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{имя}' выполнено за {ms}ms");
}
