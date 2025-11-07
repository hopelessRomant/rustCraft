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
    
}