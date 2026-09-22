fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let содержимое = "здравствуй мир замечательный мир";

    let mut карта = HashMap::new();

    for слово in содержимое.split_whitespace() {
        let счётчик = map.entry(слово).or_insert(0);
        *счётчик += 1;
    }

    println!("{карта:?}");
    // ANCHOR_END: here
}
