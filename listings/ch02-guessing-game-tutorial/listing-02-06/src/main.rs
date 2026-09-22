use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадай число!");

    let загаданное_число = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Пожалуйста, введите ваше число.");

        let mut догадка = String::new();

        io::stdin()
            .read_line(&mut догадка)
            .expect("Ошибка при чтении");

        let догадка: u32 = match догадка.trim().parse() {
            Ok(число) => число,
            Err(_) => continue,
        };

        println!("Вы угадали: {догадка}");

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
