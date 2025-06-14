fn main() {
    // ANCHOR: here
    let s = String::from("здравствуй мир");

    let здравствуй = &s[0..5];
    let world = &s[6..11];
    // ANCHOR_END: here
}
