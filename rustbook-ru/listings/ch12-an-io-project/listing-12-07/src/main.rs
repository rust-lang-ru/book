use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    // ANCHOR_END: here

    println!("Поиск значения:{}", config.query);
    println!("В файле {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Файл не доступен для чтения");

    println!("Содержимое:\n{contents}");
    // ANCHOR: here

    // --snip--
}

// --snip--

// ANCHOR_END: here
struct Config {
    query: String,
    file_path: String,
}

// ANCHOR: here
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
// ANCHOR_END: here
