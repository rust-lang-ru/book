extern crate trpl; // требуется для mdbook test

// ANCHOR: all
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut сообщения = получить_сообщения();

        while let Some(сообщение) = сообщения.next().await {
            println!("{сообщение}");
        }
    });
}

fn получить_сообщения() -> impl Stream<Предмет = String> {
    let (tx, rx) = trpl::channel();

    let сообщения ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for сообщение in сообщения {
        tx.send(format!("Сообщение: '{сообщение}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
// ANCHOR_END: all
