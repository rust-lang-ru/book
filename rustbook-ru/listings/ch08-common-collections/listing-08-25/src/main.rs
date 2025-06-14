fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let text = "здравствуй мир wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    // ANCHOR_END: here
}
