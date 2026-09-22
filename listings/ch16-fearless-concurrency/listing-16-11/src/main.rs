use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // ANCHOR: here
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let значения = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for значение in значения {
            tx1.send(значение).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let значения = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for значение in значения {
            tx.send(значение).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Получено: {получено}");
    }

    // --snip--
    // ANCHOR_END: here
}
