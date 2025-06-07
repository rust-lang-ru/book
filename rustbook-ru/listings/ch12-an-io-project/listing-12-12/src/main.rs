use std::env;
use std::fs;
use std::process;
// ANCHOR: here
use std::error::Error;

// --snip--

// ANCHOR_END: here

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Поиск значения:{}", config.query);
    println!("В файле {}", config.file_path);

    run(config);
}

// ANCHOR: here
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("Содержимое:\n{contents}");

    Ok(())
}
// ANCHOR_END: here

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("не хватает переменных");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
