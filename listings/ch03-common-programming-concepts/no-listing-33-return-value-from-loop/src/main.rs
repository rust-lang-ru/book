fn main() {
    let mut счётчик = 0;

    let итог = loop {
        счётчик += 1;

        if счётчик == 10 {
            break счётчик * 2;
        }
    };

    println!("Итог: {итог}");
}
