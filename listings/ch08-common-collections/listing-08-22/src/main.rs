fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let имя_поля = String::from("Любимый цвет");
    let значение_поля = String::from("Синий");

    let mut карта = HashMap::new();
    map.insert(имя_поля, значение_поля);
    // имя_поля и значение_поля недоступны в данном месте, попробуйте использовать их и
    // увидите какую ошибку вам выдаст сборщик!
    // ANCHOR_END: here
}
