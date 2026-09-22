use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let свойства: Vec<String> = env::args().collect();

    let (запрос, путь_до_файла) = получить_настройки(&свойства);

    // --snip--
    // ANCHOR_END: here

    println!("Поиск значения:{запрос}");
    println!("В файле {путь_до_файла}");

    let содержимое = fs::read_to_string(путь_до_файла)
        .expect("Файл не доступен для чтения");

    println!("Содержимое:\n{содержимое}");
    // ANCHOR: here
}

fn получить_настройки(свойства: &[String]) -> (&str, &str) {
    let запрос = &свойства[1];
    let путь_до_файла = &свойства[2];

    (запрос, путь_до_файла)
}
// ANCHOR_END: here
