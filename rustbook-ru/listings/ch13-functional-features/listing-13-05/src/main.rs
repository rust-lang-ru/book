fn main() {
    let mut список = vec![1, 2, 3];
    println!("До определения замыкания: {список:?}");

    let mut borrows_mutably = || список.push(7);

    borrows_mutably();
    println!("После вызова замыкания: {список:?}");
}
