use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // --snip--
    // ANCHOR_END: here

    println!("Поиск значения:{query}");
    println!("В файле {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Файл не доступен для чтения");

    println!("Содержимое:\n{contents}");
    // ANCHOR: here
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
// ANCHOR_END: here
