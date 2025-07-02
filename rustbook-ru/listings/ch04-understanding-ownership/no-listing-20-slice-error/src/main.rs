fn первое_слово(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &предмет) in bytes.iter().enumerate() {
        if предмет == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: here
fn main() {
    let mut s = String::from("здравствуй мир");

    let слово = первое_слово(&s);

    s.clear(); // error!

    println!("первое слово: {слово}");
}
// ANCHOR_END: here
