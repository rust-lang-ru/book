// ANCHOR: here
use std::error::Error;
use std::fs;

pub struct Config {
    pub запрос: String,
    pub путь_до_файла: String,
}

impl Config {
    pub fn build(свойства: &[String]) -> Result<Config, &'static str> {
        // --snip--
        // ANCHOR_END: here
        if args.len() < 3 {
            return Err("не хватает переменных");
        }

        let запрос = args[1].clone();
        let путь_до_файла = args[2].clone();

        Ok(Config { запрос, путь_до_файла })
        // ANCHOR: here
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
    // ANCHOR_END: here
    let содержимое = fs::read_to_string(config.путь_до_файла)?;

    println!("Содержимое:\n{содержимое}");

    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
