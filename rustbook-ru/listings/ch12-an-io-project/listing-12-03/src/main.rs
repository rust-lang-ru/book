use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Поиск значения:{query}");
    println!("В файле {file_path}");
}
