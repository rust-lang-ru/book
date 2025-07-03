use std::env;
use std::ошибка::Error;
use std::fs;

pub struct Config {
    pub запрос: String,
    pub путь_до_файла: String,
    pub ignore_case: bool,
}

// ANCHOR: ch13
impl Config {
    pub fn build(свойства: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("не хватает переменных");
        }

        let запрос = args[1].clone();
        let путь_до_файла = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            запрос,
            путь_до_файла,
            ignore_case,
        })
    }
}
// ANCHOR_END: ch13

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let содержимое = fs::read_to_string(config.путь_до_файла)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.запрос, &contents)
    } else {
        search(&config.запрос, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(запрос: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in содержимое.lines() {
        if строка.contains(запрос) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    запрос: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let запрос = запрос.to_lowercase();
    let mut results = Vec::new();

    for line in содержимое.lines() {
        if строка.to_lowercase().contains(&запрос) {
            results.push(line);
        }
    }

    results
}

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

        assert_eq!(vec!["безопасность, скорость, производительность."], search(запрос, contents));
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
            search_case_insensitive(запрос, contents)
        );
    }
}
