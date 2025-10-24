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

    fn will_fit(&self, test: &Rectangle) -> bool {
        self.width > test.width && self.length > test.length
    }

    fn sq(size: u32) -> Rectangle {
        Self { length: size, 
            width: size, 
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 10,
        width: 20,
    };

    let rect2 = Rectangle {
        length: 5,
        width: 10,
    };

    let sq1 = Rectangle::sq(20);

    println!("area of the rectangle is : {}", rect1.area());
    dbg!(&rect1);
    println!("will rect2 fit in rect1: {}", rect1.will_fit(&rect2));
    dbg!(&sq1);
    println!("will sq1 fit in rect1: {}", rect1.will_fit(&sq1));
}

