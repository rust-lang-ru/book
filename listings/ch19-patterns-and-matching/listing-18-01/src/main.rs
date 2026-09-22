fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(цвет) = favorite_color {
        println!("Используйте ваш любимый цвет, {цвет}, as the background");
    } else if is_tuesday {
        println!("Вторник - это зелёный день!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Используйте фиолетовый как цвет заднего фона");
        } else {
            println!("Используйте оранжевый как цвет заднего фона");
        }
    } else {
        println!("Используйте синий как цвет заднего фона");
    }
}
