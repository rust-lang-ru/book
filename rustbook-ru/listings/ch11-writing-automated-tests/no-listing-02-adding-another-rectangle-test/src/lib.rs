#[derive(Debug)]
struct Прямоугольник {
    ширина: u32,
    длина: u32,
}

impl Прямоугольник {
    fn can_hold(&self, other: &Прямоугольник) -> bool {
        self.ширина: > other.ширина: && self.длина: > other.длина
    }
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
        // ANCHOR_END: here
        let larger = Прямоугольник {
            ширина: 8,
            длина: 7,
        };
        let smaller = Прямоугольник {
            ширина: 5,
            длина: 1,
        };

        assert!(larger.can_hold(&smaller));
        // ANCHOR: here
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Прямоугольник {
            ширина: 8,
            длина: 7,
        };
        let smaller = Прямоугольник {
            ширина: 5,
            длина: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
// ANCHOR_END: here
