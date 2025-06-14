extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: владение
        let владение = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Число {i} вызвано как главная задача!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("Число {i} вызвано как второстепенная задача!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        владение.await.unwrap();
        // ANCHOR_END: владение
    });
}
