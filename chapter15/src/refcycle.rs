use std::cell::RefCell;
use std::rc::Rc;
use crate::refcycle::List::{Nil, Cons};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

// won't recomment running this code.
pub fn overflow() {
    let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));

    println!("Rc count of a: {}",Rc::strong_count(&a)); // answer will be 2

    // creating a cyclic reference using a mutible borrow of a and linking it to b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("Rc count of b: {}", Rc::strong_count(&b)); // answer will be 2
    println!("This code will leave behind uncleaned storage in the Heap and overflow the stack: {:#?}", a);
}