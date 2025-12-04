pub fn reff() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

pub fn type_inf() {
    let eg = |x| x;
    let _s = eg("closure example".to_string());
    // let n = eg(3); // eg is already labelled as a String to not integer allowed now.
}