extern crate trpl; // требуется для mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: many-messages
        let (tx, mut rx) = trpl::channel();

        let значения = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for значение in значения {
            tx.send(значение).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(значение) = rx.recv().await {
            println!("получено  '{значение}'");
        }
        // ANCHOR_END: many-messages
    });
}
