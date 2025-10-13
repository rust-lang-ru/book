extern crate trpl; // требуется для mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // ANCHOR: here
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' запущено.");
            медленно("a", 30);
            trpl::sleep(one_ms).await;
            медленно("a", 10);
            trpl::sleep(one_ms).await;
            медленно("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' окончено.");
        };

        let b = async {
            println!("'b' запущено.");
            медленно("b", 75);
            trpl::sleep(one_ms).await;
            медленно("b", 10);
            trpl::sleep(one_ms).await;
            медленно("b", 15);
            trpl::sleep(one_ms).await;
            медленно("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' окончено.");
        };
        // ANCHOR_END: here

        trpl::race(a, b).await;
    });
}

fn медленно(имя: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{имя}' выполнено за {ms}ms");
}
