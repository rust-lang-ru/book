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

    for строка in search(&config.запрос, &содержимое) {
        println!("{строка}");
    }

    Ok(())
}

pub fn search<'a>(запрос: &str, содержимое: &'a str) -> Vec<&'a str> {
    let mut итоги = Vec::new();

    for строка in содержимое.lines() {
        if строка.contains(запрос) {
            итоги.push(строка);
        }
    }

    results
}

// ANCHOR: here
pub fn search_case_insensitive<'a>(
    запрос: &str,
    содержимое: &'a str,
) -> Vec<&'a str> {
    let запрос = запрос.to_lowercase();
    let mut итоги = Vec::new();

    for строка in содержимое.lines() {
        if строка.to_lowercase().contains(&запрос) {
            итоги.push(строка);
        }
    }

    results
}
// ANCHOR_END: here

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn case_sensitive() {
        let запрос = "duct";
        let содержимое = "\
Rust:
безопасность, скорость, производительность.
Выберите три.
Duct tape.";

        assert_eq!(vec!["безопасность, скорость, производительность."], search(запрос, содержимое));
    }

    #[test]
    fn case_insensitive() {
        let запрос = "rUsT";
        let содержимое = "\
Rust:
безопасность, скорость, производительность.
Выберите три.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(запрос, содержимое)
        );
    }
}
