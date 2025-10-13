#[cfg(test)]
mod проверки {
    // ANCHOR: here
    #[test]
    fn iterator_sum() {
        let ряд_1 = vec![1, 2, 3];

        let ряд_1_перебор = ряд_1.iter();

        let всего: i32 = ряд_1_перебор.sum();

        assert_eq!(всего, 6);
    }
    // ANCHOR_END: here
}
