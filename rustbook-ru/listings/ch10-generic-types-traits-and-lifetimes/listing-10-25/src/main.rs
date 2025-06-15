// ANCHOR: here
fn первое_слово(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &предмет) in bytes.iter().enumerate() {
        if предмет == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// ANCHOR_END: here

fn main() {
    let моя_строка = String::from("здравствуй мир");

    // первое_слово works on slices of `String`s
    let слово = первое_слово(&моя_строка[..]);

    let моя_строка_literal = "здравствуй мир";

    // первое_слово works on slices of string literals
    let слово = первое_слово(&моя_строка_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let слово = первое_слово(моя_строка_literal);
}
