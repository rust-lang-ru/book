pub fn greeting(имя: &str) -> String {
    format!("Здравствуй {имя}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let итог = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
