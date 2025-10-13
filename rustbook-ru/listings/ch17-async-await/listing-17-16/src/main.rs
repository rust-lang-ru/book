extern crate trpl; // требуется для mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let значения = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for значение in значения {
                tx1.send(значение).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
        };

        let rx_fut = async {
            while let Some(значение) = rx.recv().await {
                println!("получено  '{значение}'");
            }
        };

        let tx_fut = async move {
            let значения = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for значение in значения {
                tx.send(значение).unwrap();
                trpl::sleep(Duration::from_secs(1)).await;
            }
        };

        // ANCHOR: here
        let futures =
            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

        trpl::join_all(futures).await;
        // ANCHOR_END: here
    });
}
