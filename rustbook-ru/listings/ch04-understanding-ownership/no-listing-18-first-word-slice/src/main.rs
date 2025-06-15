// ANCHOR: here
fn первое_слово(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &предмет) in bytes.iter().enumerate() {
        if предмет == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// ANCHOR_END: here

fn main() {}
