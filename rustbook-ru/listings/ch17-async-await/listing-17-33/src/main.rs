extern crate trpl; // required for mdbook test

// ANCHOR: all
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages = get_messages();

        while let Some(сообщение) = messages.next().await {
            println!("{сообщение}");
        }
    });
}

fn get_messages() -> impl Stream<Предмет = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for сообщение in messages {
        tx.send(format!("Сообщение: '{сообщение}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
// ANCHOR_END: all
