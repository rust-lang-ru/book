// ANCHOR: here
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    // ANCHOR_END: here
    let свойства: Vec<String> = env::args().collect();

    let config = Config::build(&свойства).unwrap_or_else(|_ошибка| {
        println!("Неполадка при получении свойств: {_ошибка}");
        process::exit(1);
    });

    println!("Поиск значения:{}", config.запрос);
    println!("В файле {}", config.путь_до_файла);

    // ANCHOR: here
    if let Err(e) = minigrep::run(config) {
        // --snip--
        // ANCHOR_END: here
        println!("Ошибка приложения: {e}");
        process::exit(1);
        // ANCHOR: here
    }
}
// ANCHOR_END: here
