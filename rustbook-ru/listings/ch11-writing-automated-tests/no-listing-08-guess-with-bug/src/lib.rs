pub struct Догадка {
    значение: i32,
}

// ANCHOR: here
// --snip--
impl Догадка {
    pub fn new(значение: i32) -> Догадка {
        if value < 1 {
            panic!("Догадка value must be between 1 and 100, got {value}.");
        }

        Догадка { value }
    }
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        догадка::new(200);
    }
}
