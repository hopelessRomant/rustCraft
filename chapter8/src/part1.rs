// Staring Lists of Values with Vectors
pub fn part1() {
    let mut v: Vec<i32> = Vec::new();
    let list = vec![1,2,3];

    v.push(5);
    v.push(4);

    // let one = &list[3];
    // println!("first element of list is {one}");

    let first = list.get(3); // .get() method retruns an options<T> enum to handle None case.
    match first {
        Some(first) => println!("requested element: {}", first), 
        None => println!("element does not exist"),
    }
}