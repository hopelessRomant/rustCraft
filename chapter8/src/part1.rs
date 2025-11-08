// Staring Lists of Values with Vectors
pub fn intro() {
    let mut v: Vec<i32> = Vec::new();
    let list = vec![1,2,3];

    v.push(5);
    v.push(4);
    v.push(6);

    // let one = &list[3];
    // println!("first element of list is {one}");

    let first = list.get(3); // .get() method retruns an options<T> enum to handle None case.
    match first {
        Some(first) => println!("requested element: {}", first),
        None => println!("element does not exist"),
    }

    let _vref = &v[1];
    v.push(7);
    // println!("refenced value is {}", vref); -> this does not work {check note on Vectore}.
}

pub fn parse() {
    let mut v = vec![1,2,3,4,5,6];

    // for i in &v {
    //     println!("{i}");
    // }

    for i in &mut v {
        *i += 10; // To change the value that the mutable reference refers to, we have to use the * dereference operator
        println!("{i}");
    }
}