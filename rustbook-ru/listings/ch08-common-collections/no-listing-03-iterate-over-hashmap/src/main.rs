fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Голубой"), 10);
    scores.insert(String::from("Жёлтый"), 50);

    for (ключ, значение) in &scores {
        println!("{ключ}: {значение}");
    }
    // ANCHOR_END: here
}
