use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("До определения замыкания: {list:?}");

    thread::spawn(move || println!("Из основного потока: {list:?}"))
        .join()
        .unwrap();
}
