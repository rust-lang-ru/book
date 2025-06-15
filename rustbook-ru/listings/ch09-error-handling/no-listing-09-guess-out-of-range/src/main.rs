use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадай число!");

    let загаданное_число = rand::thread_rng().gen_range(1..=100);

    // ANCHOR: here
    loop {
        // --snip--

        // ANCHOR_END: here
        println!("Пожалуйста, введите ваше число.");

        let mut догадка = String::new();

        io::stdin()
            .read_line(&mut догадка)
            .expect("Ошибка при чтении");

        // ANCHOR: here
        let догадка: i32 = match догадка.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if догадка < 1 || догадка > 100 {
            println!("Загаданное число will be between 1 and 100.");
            continue;
        }

        match догадка.cmp(&загаданное_число) {
            // --snip--
            // ANCHOR_END: here
            Ordering::Less => println!("Слишком мало!"),
            Ordering::Greater => println!("Слишком много!"),
            Ordering::Equal => {
                println!("Вы выиграли!");
                break;
            }
        }
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
