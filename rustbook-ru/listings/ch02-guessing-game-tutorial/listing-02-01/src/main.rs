// ANCHOR: all
// ANCHOR: io
use std::io;
// ANCHOR_END: io

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Угадай число!");

    println!("Пожалуйста, введите ваше число.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut догадка = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    io::stdin()
        .read_line(&mut догадка)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Ошибка при чтении");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("Вы угадали: {}", догадка);
    // ANCHOR_END: print_guess
}
// ANCHOR: all
