use std::thread;

pub fn reff() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut only_borrows = || list.push(5);
    // the scope of the mutable refference is from the declaration to the last closure call.
    // println!("Before calling closure: {list:?}"); // immutable borrow not allowe after mutable borrow
    only_borrows();
    println!("After calling closure: {list:?}");
}

pub fn type_inf() {
    let eg = |x| x;
    let _s = eg("closure example".to_string());
    // let n = eg(3); // eg is already labelled as a String to not integer allowed now.
}

// 'move' keyword to force transfer ownership into the closure, example case: threads.
pub fn thread() {
    let list = vec![2,3,5,7];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
