fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut очки = HashMap::new();

    очки.insert(String::from("Синий"), 10);
    очки.insert(String::from("Жёлтый"), 50);

    for (ключ, значение) in &очки {
        println!("{ключ}: {значение}");
    }
    // ANCHOR_END: here
}
