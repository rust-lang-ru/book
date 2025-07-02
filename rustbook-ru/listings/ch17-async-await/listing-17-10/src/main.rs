extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: many-messages
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(значение) = rx.recv().await {
            println!("received '{значение}'");
        }
        // ANCHOR_END: many-messages
    });
}
