struct МойКороб<T>(T);

impl<T> МойКороб<T> {
    fn new(x: T) -> МойКороб<T> {
        МойКороб(x)
    }
}

// ANCHOR: here
fn main() {
    let x = 5;
    let y = МойКороб::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
// ANCHOR_END: here
