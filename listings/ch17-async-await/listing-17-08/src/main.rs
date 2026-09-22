extern crate trpl; // требуется для mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: join
        let fut1 = async {
            for i in 1..10 {
                println!("Число {i} вызвано как главная задача!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("Число {i} вызвано как второстепенная задача!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
        // ANCHOR_END: join
    });
}
