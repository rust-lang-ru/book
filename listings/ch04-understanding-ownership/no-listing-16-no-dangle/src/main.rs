fn main() {
    let string = no_dangle();
}

// ANCHOR: here
fn no_dangle() -> String {
    let s = String::from("здравствуй");

    s
}
// ANCHOR_END: here
