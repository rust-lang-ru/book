use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let значение = String::from("hi");
        tx.send(значение).unwrap();
    });

    let получено = rx.recv().unwrap();
    println!("Получено: {получено}");
}
