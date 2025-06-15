pub fn add(left: uразмер, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let итог = add(2, 2);
        assert_eq!(result, 4);
    }
}
