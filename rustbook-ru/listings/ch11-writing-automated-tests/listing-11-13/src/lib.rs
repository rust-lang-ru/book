pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

fn internal_adder(left: uразмер, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let итог = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
