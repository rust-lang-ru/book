// ANCHOR: here
use std::ops::Deref;

impl<T> Deref for МойКороб<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// ANCHOR_END: here

struct МойКороб<T>(T);

impl<T> МойКороб<T> {
    fn new(x: T) -> МойКороб<T> {
        МойКороб(x)
    }
}

fn main() {
    let x = 5;
    let y = МойКороб::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
