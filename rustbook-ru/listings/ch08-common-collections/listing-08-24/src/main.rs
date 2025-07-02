fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Голубой"), 10);

    scores.entry(String::from("Жёлтый")).or_insert(50);
    scores.entry(String::from("Голубой")).or_insert(50);

    println!("{scores:?}");
    // ANCHOR_END: here
}
