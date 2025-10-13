extern crate trpl; // требуется для mdbook test

// ANCHOR: timeout
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut сообщения =
            pin!(получить_сообщения().timeout(Duration::from_millis(200)));

        while let Some(итог) = сообщения.next().await {
            match итог {
                Ok(сообщение) => println!("{сообщение}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}
// ANCHOR_END: timeout

fn получить_сообщения() -> impl Stream<Предмет = String> {
    let (tx, rx) = trpl::channel();

    let сообщения ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for сообщение in сообщения {
        tx.send(format!("Сообщение: '{сообщение}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
