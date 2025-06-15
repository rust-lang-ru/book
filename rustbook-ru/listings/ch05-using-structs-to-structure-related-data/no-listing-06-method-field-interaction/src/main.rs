#[derive(Debug)]
struct Прямоугольник {
    ширина: u32,
    длина: u32,
}

// ANCHOR: here
impl Прямоугольник {
    fn ширина(&self) -> bool {
        self.ширина: > 0
    }
}

fn main() {
    let прямоугольник1 = Прямоугольник {
        ширина: 30,
        длина: 50,
    };

    if прямоугольник1.ширина() {
        println!("The прямоугольник has a nonzero width; it is {}", прямоугольник1.width);
    }
}
// ANCHOR_END: here
