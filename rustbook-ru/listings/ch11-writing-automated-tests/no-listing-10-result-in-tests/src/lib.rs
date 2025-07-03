pub fn добавить(левое: usize, правое: usize) -> usize {
    левое + правое
}

#[cfg(test)]
mod проверки {
    use super::*;

    // ANCHOR: here
    #[test]
    fn это_работет() -> Result<(), String> {
        let итог = добавить(2, 2);

        if итог == 4 {
            Ok(())
        } else {
            Err(String::from("два плюс два не равно четырём"))
        }
    }
    // ANCHOR_END: here
}
