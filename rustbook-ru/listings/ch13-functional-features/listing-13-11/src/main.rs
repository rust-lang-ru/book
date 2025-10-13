fn main() {
    // ANCHOR: here
    let ряд_1 = vec![1, 2, 3];

    let ряд_1_перебор = ряд_1.iter();

    for значение in ряд_1_перебор {
        println!("Получено: {значение}");
    }
    // ANCHOR_END: here
}
