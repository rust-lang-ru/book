fn main() {
    // ANCHOR: here
    let числа = (2, 4, 8, 16, 32);

    match числа {
        (first, _, третье, _, fifth) => {
            println!("Некоторые числа: {первый}, {третье}, {fifth}")
        }
    }
    // ANCHOR_END: here
}
