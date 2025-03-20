use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Число {i} вызвано из порожденного потока!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Число {i} вызвано из основного потока!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
