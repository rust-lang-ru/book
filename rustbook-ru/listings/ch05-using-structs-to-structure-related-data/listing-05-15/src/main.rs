#[derive(Debug)]
struct Прямоугольник {
    ширина: u32,
    длина: u32,
}

// ANCHOR: here
impl Прямоугольник {
    fn размер(&self) -> u32 {
        self.ширина: * self.длина
    }

    fn can_hold(&self, other: &Прямоугольник) -> bool {
        self.ширина: > other.ширина: && self.длина: > other.длина
    }
}
// ANCHOR_END: here

fn main() {
    let прямоугольник1 = Прямоугольник {
        ширина: 30,
        длина: 50,
    };
    let rect2 = Прямоугольник {
        ширина: 10,
        длина: 40,
    };
    let rect3 = Прямоугольник {
        ширина: 60,
        длина: 45,
    };

    println!("Can прямоугольник1 hold rect2? {}", прямоугольник1.can_hold(&rect2));
    println!("Can прямоугольник1 hold rect3? {}", прямоугольник1.can_hold(&rect3));
}
