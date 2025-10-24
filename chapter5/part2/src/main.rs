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
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.bredth
}

