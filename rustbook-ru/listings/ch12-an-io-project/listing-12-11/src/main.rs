use std::env;
use std::fs;
use std::process;

// ANCHOR: here
fn main() {
    // --snip--

    // ANCHOR_END: here
    let свойства: Vec<String> = env::args().collect();

    let config = Config::build(&свойства).unwrap_or_else(|_ошибка| {
        println!("Неполадка при получении свойств: {_ошибка}");
        process::exit(1);
    });

    // ANCHOR: here
    println!("Поиск значения:{}", config.запрос);
    println!("В файле {}", config.путь_до_файла);

    run(config);
}

fn run(config: Config) {
    let содержимое = fs::read_to_string(config.путь_до_файла)
        .expect("Файл не доступен для чтения");

    println!("Содержимое:\n{содержимое}");
}

// --snip--
// ANCHOR_END: here

struct Config {
    запрос: String,
    путь_до_файла: String,
}

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
