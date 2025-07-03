extern crate trpl; // required for mdbook test

use std::{pin::pin, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages =
            pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(итог) = messages.next().await {
            match итог {
                Ok(сообщение) => println!("{сообщение}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

fn get_messages() -> impl Stream<Предмет = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, сообщение) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Сообщение: '{сообщение}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}

// ANCHOR: intervals
fn get_intervals() -> impl Stream<Предмет = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut счётчик = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            счётчик += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
// ANCHOR_END: intervals
