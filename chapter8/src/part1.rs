// Staring Lists of Values with Vectors
pub fn intro() {
    let mut v: Vec<i32> = Vec::new();
    let list = vec![1,2,3];

    v.push(5);
    v.push(4);
    v.push(6);

    // let one = &list[3];
    // println!("first element of list is {one}");

    let ele = list.get(1); // .get() method retruns an options<T> enum to handle None case.
    match ele {
        Some(ele) => println!("requested element: {}", ele),
        None => println!("element does not exist"),
    }

    let _vref = &v[1];
    v.push(7);
    // println!("refenced value is {}", vref); -> this does not work {check note on Vectore}.

    v.clear();
    let last = v.pop();
    match last {
        Some(last) => println!("poped element is: {}", last),
        None => println!("vector is empty"),
    }
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

#[derive(Debug)]
enum Collection {
    Int(i32),
    Text(String),
    Bool(bool),
}

// Using enums we can store different types of elements in the same vector
pub fn enums() {
    let table = vec![
        Collection::Int(25),
        Collection::Text(String::from("sahil")),
        Collection::Bool(true),
    ];

    for i in &table {
        println!("{:?}", i)
    }
}