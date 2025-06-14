// ANCHOR: here
struct МойКороб<T>(T);

impl<T> МойКороб<T> {
    fn new(x: T) -> МойКороб<T> {
        МойКороб(x)
    }
}
// ANCHOR_END: here

fn main() {}
