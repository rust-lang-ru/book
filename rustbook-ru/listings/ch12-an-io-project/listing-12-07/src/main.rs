use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let свойства: Vec<String> = env::args().collect();

    let config = Config::new(&свойства);
    // ANCHOR_END: here

    println!("Поиск значения:{}", config.запрос);
    println!("В файле {}", config.путь_до_файла);

    let содержимое = fs::read_to_string(config.путь_до_файла)
        .expect("Файл не доступен для чтения");

    println!("Содержимое:\n{содержимое}");
    // ANCHOR: here

    // --snip--
}

// --snip--

// ANCHOR_END: here
struct Config {
    запрос: String,
    путь_до_файла: String,
}

// ANCHOR: here
impl Config {
    fn new(свойства: &[String]) -> Config {
        let запрос = args[1].clone();
        let путь_до_файла = args[2].clone();

        Config { запрос, путь_до_файла }
    }
}
// ANCHOR_END: here
