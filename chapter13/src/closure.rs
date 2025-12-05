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
    // let n = eg(3); // eg is already labelled as a String, so no integer allowed now.
}

// 'move' keyword to force transfer ownership into the closure, example case: threads.
pub fn thread() {
    let list = vec![2,3,5,7];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}
// FnMut trait does not allow to move any value out of the environment 
pub fn fnmut_trait() {
    let mut list = [
        Rectangle {width: 10, height: 11},
        Rectangle {width: 11, height: 15},
        Rectangle {width: 8, height: 14},
    ];

    let mut _sort_operations: Vec<String> = Vec::new();
    let _value = String::from("closure called");

    let mut sort_ops = 0;
    list.sort_by_key(|r| {
        // sort_operations.push(value); // value cannot be moved out of the environment since sort_by_key is implimented on FnMut trait.
        sort_ops +=1;
        r.width
    });
    println!("{list:#?}\nsorted in {sort_ops} operations");
}
