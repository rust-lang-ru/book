fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut очки = HashMap::new();

    очки.insert(String::from("Синий"), 10);
    очки.insert(String::from("Синий"), 25);

    println!("{очки:?}");
    // ANCHOR_END: here
}
