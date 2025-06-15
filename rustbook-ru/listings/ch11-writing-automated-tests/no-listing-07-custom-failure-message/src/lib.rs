pub fn greeting(имя: &str) -> String {
    String::from("Здравствуй!")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ANCHOR: here
    #[test]
    fn greeting_contains_name() {
        let итог = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{итог}`"
        );
    }
    // ANCHOR_END: here
}
