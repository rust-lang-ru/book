fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut очки = HashMap::new();

    очки.insert(String::from("Синий"), 10);
    очки.insert(String::from("Жёлтый"), 50);

    let имя_игрока = String::from("Синий");
    let score = очки.get(&имя_игрока).copied().unwrap_or(0);
    // ANCHOR_END: here
}
