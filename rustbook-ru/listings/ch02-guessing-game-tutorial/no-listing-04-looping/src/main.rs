use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадай число!");

    let загаданное_число = rand::thread_rng().gen_range(1..=100);

    // ANCHOR: here
    // --snip--

    println!("Загаданное число: {загаданное_число}");

    loop {
        println!("Пожалуйста, введите ваше число.");

        // --snip--

        // ANCHOR_END: here

        let mut догадка = String::new();

        io::stdin()
            .read_line(&mut догадка)
            .expect("Ошибка при чтении");

        let догадка: u32 = догадка.trim().parse().expect("Пожалуйста, введите число!");

        println!("Вы угадали: {догадка}");

        // ANCHOR: here
        match догадка.cmp(&загаданное_число) {
            Ordering::Less => println!("Слишком мало!"),
            Ordering::Greater => println!("Слишком много!"),
            Ordering::Equal => println!("Вы выиграли!"),
        }
    }
}
// ANCHOR_END: here
