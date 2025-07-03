fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the значение {a}");
    10
}

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let значение = prints_and_returns_10(4);
        assert_eq!(значение, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let значение = prints_and_returns_10(8);
        assert_eq!(значение, 5);
    }
}
