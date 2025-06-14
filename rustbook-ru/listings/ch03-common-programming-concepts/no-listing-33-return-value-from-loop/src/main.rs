fn main() {
    let mut счётчик = 0;

    let result = loop {
        счётчик += 1;

        if счётчик == 10 {
            break счётчик * 2;
        }
    };

    println!("The result is {result}");
}
