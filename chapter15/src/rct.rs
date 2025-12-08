// use crate::boxt::List::{Cons, Nil}; // This is not desirable since we nned multiple owners

use std::rc::Rc;

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil
}

use crate::rct::RcList::{Cons, Nil};

pub fn multiple_owner() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil))))); // make a the smart pointer
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a)); // calling Rc::clone instead of a.clone to not make copies of data in heap.
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}       
