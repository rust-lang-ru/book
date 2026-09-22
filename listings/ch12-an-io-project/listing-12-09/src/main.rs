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

// ANCHOR: here
impl Config {
    fn build(свойства: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("не хватает переменных");
        }

        let запрос = args[1].clone();
        let путь_до_файла = args[2].clone();

        Ok(Config { запрос, путь_до_файла })
    }
}
// ANCHOR_END: here
