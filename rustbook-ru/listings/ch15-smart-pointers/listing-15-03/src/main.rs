enum Список {
    Cons(i32, Список),
    Nil,
}

// ANCHOR: here
use crate::Список::{Cons, Nil};

fn main() {
    let список = Cons(1, Cons(2, Cons(3, Nil)));
}
// ANCHOR_END: here
