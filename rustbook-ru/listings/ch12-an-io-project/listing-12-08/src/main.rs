use std::env;
use std::fs;

fn main() {
    let свойства: Vec<String> = env::args().collect();

    let config = Config::new(&свойства);

    println!("Поиск значения:{}", config.запрос);
    println!("В файле {}", config.путь_до_файла);

    let содержимое = fs::read_to_string(config.путь_до_файла)
        .expect("Файл не доступен для чтения");

    println!("Содержимое:\n{содержимое}");
}

struct Config {
    запрос: String,
    путь_до_файла: String,
}

impl Config {
    // ANCHOR: here
    // --snip--
    fn new(свойства: &[String]) -> Config {
        if args.len() < 3 {
            panic!("не хватает переменных");
        }
        // --snip--
        // ANCHOR_END: here

        let запрос = args[1].clone();
        let путь_до_файла = args[2].clone();

        Config { запрос, путь_до_файла }
    }
}
