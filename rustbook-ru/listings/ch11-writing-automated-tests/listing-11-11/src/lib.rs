pub fn добвить_второе(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn добвить_второе_and_two() {
        let итог = добвить_второе(2);
        assert_eq!(итог, 4);
    }

    #[test]
    fn add_three_and_two() {
        let итог = добвить_второе(3);
        assert_eq!(итог, 5);
    }

    #[test]
    fn one_hundred() {
        let итог = добвить_второе(100);
        assert_eq!(итог, 102);
    }
}
