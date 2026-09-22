pub fn добвить_второе(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn это_добавляет_второе() {
        let итог = добвить_второе(2);
        assert_eq!(итог, 4);
    }
}
