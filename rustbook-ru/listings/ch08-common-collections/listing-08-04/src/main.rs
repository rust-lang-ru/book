fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let третий: &i32 = &v[2];
    println!("Третье значение {третий}");

    let третий: Option<&i32> = v.get(2);
    match третий {
        Some(третий) => println!("Третье значение {третий}"),
        None => println!("Это не третье значение."),
    }
    // ANCHOR_END: here
}
