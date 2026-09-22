use std::error::Error;
use std::fs;

pub struct Config {
    pub запрос: String,
    pub путь_до_файла: String,
}

impl Config {
    pub fn build(свойства: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("не хватает переменных");
        }

        let запрос = args[1].clone();
        let путь_до_файла = args[2].clone();

        Ok(Config { запрос, путь_до_файла })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let содержимое = fs::read_to_string(config.путь_до_файла)?;

    Ok(())
}

// ANCHOR: here
pub fn search<'a>(запрос: &str, содержимое: &'a str) -> Vec<&'a str> {
    vec![]
}
// ANCHOR_END: here

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn один_итог() {
        let запрос = "duct";
        let содержимое = "\
Rust:
безопасность, скорость, производительность.
Выберите три.";

        assert_eq!(vec!["безопасность, скорость, производительность."], search(запрос, содержимое));
    }
}
