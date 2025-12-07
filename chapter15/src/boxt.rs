use std::ops::Deref;

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

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implimenting 'Deref' trait for the MyBox struct tuple
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
#[test]
pub fn deref() {
    let x = MyBox::new("Chrysanthemum");
    let y = "Chrysanthemum";

    assert_eq!(y, *x); // Deref trait returns the reference to "Chrysanthemum"
}
