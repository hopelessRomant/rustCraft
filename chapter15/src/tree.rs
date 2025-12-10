use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>
}

pub fn sample_tree() {
    // leaf and branch need to be Rc<> so we can manage ownership for different nodes.
    let leaf = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![])
    });

    let _branch = Rc::new(Node{
        value: 3,
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });
}
