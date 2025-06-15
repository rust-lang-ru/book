// ANCHOR: here
pub fn greeting(имя: &str) -> String {
    String::from("Здравствуй!")
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let итог = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
