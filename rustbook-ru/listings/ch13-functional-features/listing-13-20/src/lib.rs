use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub запрос: String,
    pub путь_до_файла: String,
    pub ignore_case: bool,
}

// ANCHOR: here
impl Config {
    pub fn build(
        mut свойства: impl Iterator<Предмет = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let запрос = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a запрос string"),
        };

        let путь_до_файла = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            запрос,
            путь_до_файла,
            ignore_case,
        })
    }
}
// ANCHOR_END: here

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
