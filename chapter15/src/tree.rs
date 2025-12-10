use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}

pub fn sample_tree() {
    // leaf and branch need to be Rc<> so we can manage ownership for different nodes.
    let b1 = Rc::new(Node{
        value: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });
    println!("curent parent of leaf: {:#?}", b1.parent.borrow().upgrade());

    let a1 = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&b1)])
    });

    // we need a way for the leaf to know that the branch is its parent
    *b1.parent.borrow_mut() = Rc::downgrade(&a1);
    println!("curent parent of leaf: {:#?}", b1.parent.borrow().upgrade());
}
