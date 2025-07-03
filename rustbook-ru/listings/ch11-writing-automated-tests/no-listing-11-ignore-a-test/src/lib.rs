pub fn добавить(левое: usize, правое: usize) -> usize {
    левое + правое
}

// ANCHOR: here
#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn это_работет() {
        let итог = добавить(2, 2);
        assert_eq!(итог, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
// ANCHOR_END: here
