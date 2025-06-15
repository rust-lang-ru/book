fn main() {
    // ANCHOR: here
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, третий, _, fifth) => {
            println!("Some numbers: {первый}, {третий}, {fifth}")
        }
    }
    // ANCHOR_END: here
}
