#[derive(Debug)]
struct Прямоугольник {
    ширина: u32,
    длина: u32,
}

// ANCHOR: here
impl Прямоугольник {
    fn квадрат(размер: u32) -> Self {
        Self {
            ширина: размер,
            длина: размер,
        }
    }
}
// ANCHOR_END: here

fn main() {
    let sq = Прямоугольник::квадрат(3);
}
