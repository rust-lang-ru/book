fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let имя_поля = String::from("Любимый цвет");
    let значение_поля = String::from("Голубой");

    let mut map = HashMap::new();
    map.insert(имя_поля, значение_поля);
    // имя_поля and значение_поля are invalid at this point, try using them and
    // see what compiler error you get!
    // ANCHOR_END: here
}
