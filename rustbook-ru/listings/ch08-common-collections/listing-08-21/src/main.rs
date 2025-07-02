fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Голубой"), 10);
    scores.insert(String::from("Жёлтый"), 50);

    let team_name = String::from("Голубой");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // ANCHOR_END: here
}
