enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Сообщение {
    Выход,
    Move { x: i32, y: i32 },
    Write(String),
    Смена_цвета(Color),
}

fn main() {
    let msg = Сообщение::Смена_цвета(Color::Hsv(0, 160, 255));

    match msg {
        Сообщение::Смена_цвета(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Сообщение::Смена_цвета(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
