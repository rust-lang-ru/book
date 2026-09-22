fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut очки = HashMap::new();
    очки.insert(String::from("Синий"), 10);

    очки.entry(String::from("Жёлтый")).or_insert(50);
    очки.entry(String::from("Синий")).or_insert(50);

    println!("{очки:?}");
    // ANCHOR_END: here
}
