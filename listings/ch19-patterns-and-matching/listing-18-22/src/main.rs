fn main() {
    // ANCHOR: here
    let s = Some(String::from("Здравствуй!"));

    if let Some(_) = s {
        println!("найдено a string");
    }

    println!("{s:?}");
    // ANCHOR_END: here
}
