use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let владение = thread::spawn(|| {
        println!("Это вектор: {v:?}");
    });

    drop(v); // Только не это!

    владение.join().unwrap();
}
