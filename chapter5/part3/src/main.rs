// using methods to calculate area of reactangle

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 10,
        width: 20,
    };
     println!("area of the rectangle is : {}", rect1.area())
}
