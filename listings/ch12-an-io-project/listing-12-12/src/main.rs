use std::env;
use std::fs;
use std::process;
// ANCHOR: here
use std::error::Error;

// --snip--

// ANCHOR_END: here

fn main() {
    let свойства: Vec<String> = env::args().collect();

    let config = Config::build(&свойства).unwrap_or_else(|_ошибка| {
        println!("Неполадка при получении свойств: {_ошибка}");
        process::exit(1);
    });

    println!("Поиск значения:{}", config.запрос);
    println!("В файле {}", config.путь_до_файла);

    run(config);
}

// ANCHOR: here
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let содержимое = fs::read_to_string(config.путь_до_файла)?;

    println!("Содержимое:\n{содержимое}");

    Ok(())
}
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
