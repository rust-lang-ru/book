pub struct Догадка {
    значение: i32,
}

// ANCHOR: here
// --snip--
impl Догадка {
    pub fn new(значение: i32) -> Догадка {
        if значение < 1 {
            panic!("Догадка значение между 1 и 100, полученное {значение}.");
        }

        Догадка { значение }
    }
}
// ANCHOR_END: here

#[cfg(test)]
mod проверки {
    use super::*;

    #[test]
    #[should_panic]
    fn больше_чем_100() {
        догадка::new(200);
    }
}
