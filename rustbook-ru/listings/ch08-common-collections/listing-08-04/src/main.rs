fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let третье: &i32 = &v[2];
    println!("Третье значение {третье}");

    let третье: Option<&i32> = v.get(2);
    match третье {
        Some(третье) => println!("Третье значение {третье}"),
        None => println!("Это не третье значение."),
    }
    // ANCHOR_END: here
}
