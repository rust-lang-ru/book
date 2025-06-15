pub struct Догадка {
    значение: i32,
}

impl Догадка {
    pub fn new(значение: i32) -> Догадка {
        // ANCHOR: here
        if value < 1 {
            panic!(
                "Догадка value must be less than or equal to 100, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Догадка value must be greater than or equal to 1, got {value}."
            );
        }
        // ANCHOR_END: here

        Догадка { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        догадка::new(200);
    }
}
