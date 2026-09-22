use std::env;
use std::process;

fn main() {
    let свойства: Vec<String> = env::args().collect();

    let config = Config::build(&свойства).unwrap_or_else(|_ошибка| {
        println!("Неполадка при получении свойств: {_ошибка}");
        process::exit(1);
    });

    println!("Поиск значения:{}", config.запрос);
    println!("В файле {}", config.путь_до_файла);

    if let Err(e) = run(config) {
        println!("Ошибка приложения: {e}");
        process::exit(1);
    }
}
