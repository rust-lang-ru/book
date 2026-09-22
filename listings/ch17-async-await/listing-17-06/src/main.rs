extern crate trpl; // требуется для mdbook test

// ANCHOR: all
use std::time::Duration;

fn main() {
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("Число {i} вызвано как главная задача!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("Число {i} вызвано как второстепенная задача!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
// ANCHOR_END: all
