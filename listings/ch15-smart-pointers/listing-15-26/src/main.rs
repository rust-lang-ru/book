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

// ANCHOR: here
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc счётчик = {}", Rc::strong_count(&a));
    println!("a next предмет = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc счётчик = {}", Rc::strong_count(&b));
    println!("b next предмет = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next предмет = {:?}", a.tail());
}
// ANCHOR_END: here
