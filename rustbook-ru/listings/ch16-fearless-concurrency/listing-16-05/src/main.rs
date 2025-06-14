use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let владение = thread::spawn(move || {
        println!("Это вектор: {v:?}");
    });

    владение.join().unwrap();
}
