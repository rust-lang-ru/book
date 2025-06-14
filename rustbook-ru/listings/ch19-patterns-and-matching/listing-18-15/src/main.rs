enum Сообщение {
    Выход,
    Move { x: i32, y: i32 },
    Write(String),
    Смена_цвета(i32, i32, i32),
}

fn main() {
    let msg = Сообщение::Смена_цвета(0, 160, 255);

    match msg {
        Сообщение::Выход => {
            println!("The Выход variant has no data to destructure.");
        }
        Сообщение::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Сообщение::Write(text) => {
            println!("Text сообщение: {text}");
        }
        Сообщение::Смена_цвета(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }
}
