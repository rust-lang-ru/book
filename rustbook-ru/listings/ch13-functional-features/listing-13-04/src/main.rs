fn main() {
    let list = vec![1, 2, 3];
    println!("До определения замыкания: {list:?}");

    let only_borrows = || println!("Вызов замыкания: {list:?}");

    println!("До вызова замыкания: {list:?}");
    only_borrows();
    println!("После вызова замыкания: {list:?}");
}
