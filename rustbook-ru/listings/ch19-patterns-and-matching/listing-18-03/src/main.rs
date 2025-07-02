fn main() {
    // ANCHOR: here
    let v = vec!['a', 'b', 'c'];

    for (index, значение) in v.iter().enumerate() {
        println!("{значение} is at index {указатель}");
    }
    // ANCHOR_END: here
}
