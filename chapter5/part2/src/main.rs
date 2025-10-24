#[derive(Debug)]
struct Rectangle {
    length: u32,
    bredth: u32,
}

fn main() {
    let rect1 = Rectangle {
        length: 20,
        bredth: 10,
    };
    println!("the area of the rectangle is {}", area(&rect1));

// pretty printing / debugging methods
    println!("rect1 1 is {rect1:?}");
    println!("rect1 1 is {rect1:#?}");
    dbg!(&rect1); // returns ownership, so it can be used in expressions as well
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.bredth
}

