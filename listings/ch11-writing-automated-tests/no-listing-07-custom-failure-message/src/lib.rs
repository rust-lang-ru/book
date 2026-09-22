pub fn здравствуй(имя: &str) -> String {
    String::from("Здравствуй!")
}

#[cfg(test)]
mod проверки {
    use super::*;

    // ANCHOR: here
    #[test]
    fn здравствуй_с_содержанием_имени() {
        let итог = здравствуй("Ольга");
        assert!(
            итог.contains("Ольга"),
            "Здравствуй did not contain name, значение получено `{итог}`"
        );
    }
    // ANCHOR_END: here
}
