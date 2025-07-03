use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадай число!");

    let загаданное_число = rand::thread_rng().gen_range(1..=100);

    println!("Загаданное число: {загаданное_число}");

    loop {
        println!("Пожалуйста, введите ваше число.");

        let mut догадка = String::new();

        // ANCHOR: here
        // --snip--

        io::stdin()
            .read_line(&mut догадка)
            .expect("Ошибка при чтении");

        // ANCHOR: ch19
        let догадка: u32 = match догадка.trim().parse() {
            Ok(число) => число,
            Err(_) => continue,
        };
        // ANCHOR_END: ch19

        println!("Вы угадали: {догадка}");

        // --snip--
        // ANCHOR_END: here

        match догадка.cmp(&загаданное_число) {
            Ordering::Less => println!("Слишком мало!"),
            Ordering::Greater => println!("Слишком много!"),
            Ordering::Equal => {
                println!("Вы выиграли!");
                break;
            }
        }
    }
}
