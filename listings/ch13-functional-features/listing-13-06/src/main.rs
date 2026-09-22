use std::thread;

fn main() {
    let список = vec![1, 2, 3];
    println!("До определения замыкания: {список:?}");

    thread::spawn(move || println!("Из основного потока: {список:?}"))
        .join()
        .unwrap();
}
