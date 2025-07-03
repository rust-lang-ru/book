pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn это_работет() {
        assert_eq!(3, add_one(2));
    }
}
