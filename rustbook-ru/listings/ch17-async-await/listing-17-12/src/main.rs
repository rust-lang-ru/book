extern crate trpl; // требуется для mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: with-move
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
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
        };

        let rx_fut = async {
            while let Some(значение) = rx.recv().await {
                println!("получено  '{значение}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
        // ANCHOR_END: with-move
    });
}
