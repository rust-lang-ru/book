// ANCHOR: here
use std::env;
use std::fs;

fn main() {
    // --snip--
    // ANCHOR_END: here
    let свойства: Vec<String> = env::args().collect();

    let запрос = &свойства[1];
    let путь_до_файла = &свойства[2];

    println!("Поиск значения:{запрос}");
    // ANCHOR: here
    println!("В файле {путь_до_файла}");

    let содержимое = fs::read_to_string(путь_до_файла)
        .expect("Файл не доступен для чтения");

    println!("Содержимое:\n{содержимое}");
}
// ANCHOR_END: here
