pub fn добвить_второе(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn добвить_второе_and_two() {
        assert_eq!(4, добвить_второе(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, добвить_второе(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, добвить_второе(100));
    }
}
