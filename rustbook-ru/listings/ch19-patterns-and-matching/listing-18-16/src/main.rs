enum Цвет {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Сообщение {
    Выход,
    Move { x: i32, y: i32 },
    Write(String),
    Смена_цвета(Цвет),
}

fn main() {
    let msg = Сообщение::Смена_цвета(Цвет::Hsv(0, 160, 255));

    match msg {
        Сообщение::Смена_цвета(Цвет::Rgb(r, g, b)) => {
            println!("Change цвет to красный {r}, зелёный {g}, and синий {b}");
        }
        Сообщение::Смена_цвета(Цвет::Hsv(h, s, v)) => {
            println!("Change цвет to hue {h}, saturation {s}, значение {v}")
        }
        _ => (),
    }
}
