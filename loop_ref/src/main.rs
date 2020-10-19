use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(10, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    let list = match a.tail() {
        Some(item) => item,
        None => panic!(),
    };
    *list.borrow_mut() = Rc::clone(&b);
    println!("a rc count = {}", Rc::strong_count(&a));
    println!("b rc count = {}", Rc::strong_count(&b));
    println!("a next item = {:?}", a.tail());
}
