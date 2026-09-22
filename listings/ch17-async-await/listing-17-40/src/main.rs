extern crate trpl; // требуется для mdbook test

use std::{pin::pin, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let сообщения получить_сообщения().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval #{счётчик}"))
            .throttle(Duration::from_millis(500))
            .timeout(Duration::from_secs(10));
        let merged = сообщения.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(итог) = stream.next().await {
            match итог {
                Ok(предмет) => println!("{предмет}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

// ANCHOR: errors
fn получить_сообщения() -> impl Stream<Предмет = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let сообщения ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (указатель, сообщение) in сообщения.into_iter().enumerate() {
            let time_to_sleep = if указатель % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Сообщение: '{сообщение}'")) {
                eprintln!("Cannot send сообщение '{сообщение}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Предмет = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut счётчик = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            счётчик += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {счётчик}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
// ANCHOR_END: errors
