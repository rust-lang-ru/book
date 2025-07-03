fn main() {
    // ANCHOR: here
    let mut v = vec![1, 2, 3, 4, 5];

    let первый = &v[0];

    v.push(6);

    println!("Первая переменная: {первый}");
    // ANCHOR_END: here
}
