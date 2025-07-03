fn main() {
    // ANCHOR: here
    let s = Some(String::from("Здравствуй!"));

    if let Some(_s) = s {
        println!("найдено a string");
    }

    println!("{s:?}");
    // ANCHOR_END: here
}
