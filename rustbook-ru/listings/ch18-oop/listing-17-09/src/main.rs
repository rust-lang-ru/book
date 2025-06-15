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

// ANCHOR: here
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                ширина: 75,
                длина: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                ширина: 50,
                длина: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
// ANCHOR_END: here
