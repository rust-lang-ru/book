use std::io;

fn main() {
    println!("Угадай число!");

    println!("Пожалуйста, введите ваше число.");

    let mut догадка = String::new();

    io::stdin().read_line(&mut догадка);

    println!("Вы угадали: {догадка}");
}
