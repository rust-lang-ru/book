use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: here
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|_ошибка| {
        eprintln!("Неполадка при получении свойств: {_ошибка}");
        process::exit(1);
    });

    // --snip--
    // ANCHOR_END: here

    if let Err(e) = minigrep::run(config) {
        eprintln!("Ошибка приложения: {e}");
        process::exit(1);
    }
    // ANCHOR: here
}
// ANCHOR_END: here
