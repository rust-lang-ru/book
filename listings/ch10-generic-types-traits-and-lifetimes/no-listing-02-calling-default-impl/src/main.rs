use электронная_площадка::{self, НовостнаяСтатья, КраткоеСодержание};

fn main() {
    // ANCHOR: here
    let статья = НовостнаяСтатья {
        заголовок: String::from("Penguins win the Stanley Cup Championship!"),
        местонахождение: String::from("Pittsburgh, PA, USA"),
        сочинитель: String::from("Iceburgh"),
        содержимое: String::from(
            "The Pittsburgh Penguins once again are the best \
             хоккейная команда в НХЛ.",
        ),
    };

    println!("Новая статья доступна! {}", статья.подвести_итог());
    // ANCHOR_END: here
}
