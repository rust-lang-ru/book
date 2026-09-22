#[derive(Debug)]
enum Список {
    Cons(Rc<RefCell<i32>>, Rc<Список>),
    Nil,
}

use crate::Список::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let значение = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&значение), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *значение.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
