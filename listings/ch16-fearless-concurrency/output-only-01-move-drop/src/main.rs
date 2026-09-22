use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let владение = thread::spawn(move || {
        println!("Это вектор: {v:?}");
    });

    drop(v); // oh no!

    владение.join().unwrap();
}
