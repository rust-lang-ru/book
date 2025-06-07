use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Поиск значения:{}", config.query);
    println!("В файле {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Файл не доступен для чтения");

    println!("Содержимое:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // ANCHOR: here
    // --snip--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("не хватает переменных");
        }
        // --snip--
        // ANCHOR_END: here

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
