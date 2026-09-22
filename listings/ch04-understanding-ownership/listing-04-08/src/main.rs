fn первое_слово(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &предмет) in bytes.iter().enumerate() {
        if предмет == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("здравствуй мир");

    let слово = первое_слово(&s); // слово will get значение 5

    s.clear(); // this empties the String, making it equal to ""

    // слово still has значение 5 here, but there's no more string that
    // we could meaningfully use значение 5 with. слово is now totally invalid!
}
// ANCHOR_END: here
