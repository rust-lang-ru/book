use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let свойства: Vec<String> = env::args().collect();

    let config = Config::build(&свойства).unwrap_or_else(|_ошибка| {
        println!("Неполадка при получении свойств: {_ошибка}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Ошибка приложения: {e}");
        process::exit(1);
    }
}
