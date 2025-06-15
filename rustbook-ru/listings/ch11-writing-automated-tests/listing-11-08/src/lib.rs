pub struct Догадка {
    значение: i32,
}

impl Догадка {
    pub fn new(значение: i32) -> Догадка {
        if value < 1 || value > 100 {
            panic!("Догадка value must be between 1 and 100, got {value}.");
        }

        Догадка { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        догадка::new(200);
    }
}
