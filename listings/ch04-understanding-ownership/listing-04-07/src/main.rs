// ANCHOR: here
fn первое_слово(s: &String) -> usize {
    // ANCHOR: as_bytes
    let bytes = s.as_bytes();
    // ANCHOR_END: as_bytes

    // ANCHOR: iter
    for (i, &предмет) in bytes.iter().enumerate() {
        // ANCHOR_END: iter
        // ANCHOR: inside_for
        if предмет == b' ' {
            return i;
        }
    }

    s.len()
    // ANCHOR_END: inside_for
}
// ANCHOR_END: here

fn main() {}
