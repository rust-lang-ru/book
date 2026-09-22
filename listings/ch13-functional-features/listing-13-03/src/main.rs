fn main() {
    // ANCHOR: here
    let пример_замыкания = |x| x;

    let s = пример_замыкания(String::from("здравствуй"));
    let n = пример_замыкания(5);
    // ANCHOR_END: here
}
