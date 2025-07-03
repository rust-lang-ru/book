// ANCHOR: here
pub fn здравствуй(имя: &str) -> String {
    String::from("Здравствуй!")
}
// ANCHOR_END: here

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn здравствуй_с_содержанием_имени() {
        let итог = здравствуй("Ольга");
        assert!(итог.contains("Ольга"));
    }
}
