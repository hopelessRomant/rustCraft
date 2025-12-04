pub fn reff() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut only_borrows = || list.push(5);

    // println!("Before calling closure: {list:?}"); // immutable borrow not allowe after mutable borrow
    only_borrows();
    println!("After calling closure: {list:?}");
}

pub fn type_inf() {
    let eg = |x| x;
    let _s = eg("closure example".to_string());
    // let n = eg(3); // eg is already labelled as a String to not integer allowed now.
}