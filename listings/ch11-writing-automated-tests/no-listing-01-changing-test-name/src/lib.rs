pub fn добавить(левое: usize, правое: usize) -> usize {
    левое + правое
}

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn exploration() {
        let итог = добавить(2, 2);
        assert_eq!(итог, 4);
    }
}
