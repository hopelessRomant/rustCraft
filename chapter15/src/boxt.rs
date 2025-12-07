pub fn basic() {
    let x = Box::new(5);
    let y =x;
    println!("{y}");
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use crate::boxt::List::{Cons, Nil};

pub fn cons_list() {
    let x = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{x:?}")
}
