use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: ch13
fn main() {
    let свойства: Vec<String> = env::args().collect();

    let config = Config::build(&свойства).unwrap_or_else(|_ошибка| {
        eprintln!("Неполадка при получении свойств: {_ошибка}");
        process::exit(1);
    });

    // --snip--
    // ANCHOR_END: ch13

    if let Err(e) = minigrep::run(config) {
        eprintln!("Ошибка приложения: {e}");
        process::exit(1);
    }
    // ANCHOR: ch13
}
// ANCHOR_END: ch13
