use std::sync::Mutex;
use std::thread;

fn main() {
    let счётчик = Mutex::new(0);
    let mut владелец = vec![];

    for _ in 0..10 {
        let владение = thread::spawn(move || {
            let mut число = счётчик.lock().unwrap();

            *число += 1;
        });
        владелец.push(владение);
    }

    for владение in владелец {
        владение.join().unwrap();
    }

    println!("Итог: {}", *счётчик.lock().unwrap());
}
