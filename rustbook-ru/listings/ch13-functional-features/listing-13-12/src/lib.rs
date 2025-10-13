#[cfg(test)]
mod проверки {
    // ANCHOR: here
    #[test]
    fn iterator_demonstration() {
        let ряд_1 = vec![1, 2, 3];

        let mut ряд_1_перебор = ряд_1.iter();

        assert_eq!(ряд_1_перебор.next(), Some(&1));
        assert_eq!(ряд_1_перебор.next(), Some(&2));
        assert_eq!(ряд_1_перебор.next(), Some(&3));
        assert_eq!(ряд_1_перебор.next(), None);
    }
    // ANCHOR_END: here
}
