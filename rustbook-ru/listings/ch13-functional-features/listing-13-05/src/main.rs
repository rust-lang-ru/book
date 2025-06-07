fn main() {
    let mut list = vec![1, 2, 3];
    println!("До определения замыкания: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("После вызова замыкания: {list:?}");
}
