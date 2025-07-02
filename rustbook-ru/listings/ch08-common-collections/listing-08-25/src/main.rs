fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let text = "здравствуй мир замечательный мир";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let счётчик = map.entry(word).or_insert(0);
        *счётчик += 1;
    }

    println!("{map:?}");
    // ANCHOR_END: here
}
