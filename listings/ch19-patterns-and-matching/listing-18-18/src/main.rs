fn main() {
    // ANCHOR: here
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized значение");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_значение:?}");
    // ANCHOR_END: here
}
