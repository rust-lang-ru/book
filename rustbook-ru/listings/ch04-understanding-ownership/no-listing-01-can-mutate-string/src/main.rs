fn main() {
    // ANCHOR: here
    let mut s = String::from("здравствуй");

    s.push_str(", мир!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `здравствуй мир!`
                     // ANCHOR_END: here
}
