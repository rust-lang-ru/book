use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Пожалуйста, введите указатель массива.");

    let mut указатель = String::new();

    io::stdin()
        .read_line(&mut указатель)
        .expect("Ошибка при чтении");

    let index: usize = указатель
        .trim()
        .parse()
        .expect("Введённый указатель не число");

    let element = a[указатель];

    println!("The значение of the element at index {указатель} is: {element}");
}
