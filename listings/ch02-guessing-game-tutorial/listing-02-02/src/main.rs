use std::io;

fn main() {
    println!("Угадай число!");

    println!("Пожалуйста, введите ваше число.");

    let mut догадка = String::new();

    io::stdin()
        .read_line(&mut догадка)
        .expect("Ошибка при чтении");

    println!("Вы угадали: {догадка}");
}
