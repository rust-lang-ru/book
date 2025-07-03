fn main() {
    let числа = (2, 4, 8, 16, 32);

    match числа {
        (first, .., last) => {
            println!("Некоторые числа: {первый}, {last}");
        }
    }
}
