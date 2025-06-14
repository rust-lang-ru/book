extern crate trpl; // required for mdbook test

// ANCHOR: timeout
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages =
            pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(сообщение) => println!("{сообщение}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}
// ANCHOR_END: timeout

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for сообщение in messages {
        tx.send(format!("Сообщение: '{сообщение}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
