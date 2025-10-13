use std::sync::Mutex;
use std::thread;

fn main() {
    let счётчик = Mutex::new(0);
    let mut владелец = vec![];

    let владение = thread::spawn(move || {
        let mut число = счётчик.lock().unwrap();

        *число += 1;
    });
    владелец.push(владение);

    let handle2 = thread::spawn(move || {
        let mut num2 = счётчик.lock().unwrap();

        *num2 += 1;
    });
    владелец.push(handle2);

    for владение in владелец {
        владение.join().unwrap();
    }

    println!("Итог: {}", *счётчик.lock().unwrap());
}
