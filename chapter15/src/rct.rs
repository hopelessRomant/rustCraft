// use crate::boxt::List::{Cons, Nil}; // This is not desirable since we nned multiple owners

use std::rc::Rc;

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil
}

use crate::rct::RcList::{Cons, Nil};

pub fn multiple_owner() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil))))); // make a the smart pointer
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

}
