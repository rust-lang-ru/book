use std::env;

fn main() {
    let свойства: Vec<String> = env::args().collect();

    let запрос = &свойства[1];
    let путь_до_файла = &свойства[2];

    println!("Поиск значения:{запрос}");
    println!("В файле {путь_до_файла}");
}
