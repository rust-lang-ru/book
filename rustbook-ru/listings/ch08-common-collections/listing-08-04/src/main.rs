fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let третий: &i32 = &v[2];
    println!("The третий element is {третий}");

    let третий: Option<&i32> = v.get(2);
    match третий {
        Some(третий) => println!("The третий element is {третий}"),
        None => println!("There is no третий element."),
    }
    // ANCHOR_END: here
}
