fn main() {
    let reference_to_nothing = dangle();
}

// ANCHOR: here
fn dangle() -> &String { // dangle returns ссылка to a String

    let s = String::from("здравствуй"); // s is a new String

    &s // we return ссылка to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
// ANCHOR_END: here
