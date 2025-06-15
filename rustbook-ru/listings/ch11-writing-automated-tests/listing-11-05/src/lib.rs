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
