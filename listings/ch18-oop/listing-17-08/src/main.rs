// ANCHOR: here
use gui::Draw;

struct SelectBox {
    ширина: u32,
    длина: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
// ANCHOR_END: here

fn main() {}
