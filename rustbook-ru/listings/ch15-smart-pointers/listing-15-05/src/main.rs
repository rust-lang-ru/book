enum Список {
    Cons(i32, Box<Список>),
    Nil,
}

use crate::Список::{Cons, Nil};

fn main() {
    let список = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
