pub struct Догадка {
    значение: i32,
}

impl Догадка {
    pub fn new(значение: i32) -> Догадка {
        if value < 1 || value > 100 {
            panic!("Догадка value must be between 1 и 100, got {значение}.");
        }

        Догадка { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
