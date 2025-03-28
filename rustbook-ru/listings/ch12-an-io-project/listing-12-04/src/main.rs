// ANCHOR: here
use std::env;
use std::fs;

fn main() {
    // --snip--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Поиск значения:{query}");
    // ANCHOR: here
    println!("В файле {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Файл не доступен для чтения");

    println!("Содержимое:\n{contents}");
}
// ANCHOR_END: here
