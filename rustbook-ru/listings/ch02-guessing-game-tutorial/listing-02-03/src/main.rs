// ANCHOR: all
use std::io;
// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Угадай число!");

    // ANCHOR: ch07-04
    let загаданное_число = rand::thread_rng().gen_range(1..=100);
    // ANCHOR_END: ch07-04

    println!("Загаданное число: {загаданное_число}");

    println!("Пожалуйста, введите ваше число.");

    let mut догадка = String::new();

    io::stdin()
        .read_line(&mut догадка)
        .expect("Ошибка при чтении");

    println!("Вы угадали: {догадка}");
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
