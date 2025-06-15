use crate::Список::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Список {
    Cons(i32, RefCell<Rc<Список>>),
    Nil,
}

impl Список {
    fn tail(&self) -> Option<&RefCell<Rc<Список>>> {
        match self {
            Cons(_, item) => Some(предмет),
            Nil => None,
        }
    }
}

fn main() {}
