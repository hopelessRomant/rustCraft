#[allow(dead_code)]
enum IpAdress {
    V4(u8, u8, u8, u8), 
    V6(String),
}

#[allow(dead_code)]
enum Exmaples {
    Point {x: i32, y:i32, z:i32}, // defining struct like variants in enum
    Color (i32, i32, i32),
    Exit,
}

// methods on enum
impl Exmaples {
    fn dot (&self) -> (&i32,&i32,&i32) {
        if let Exmaples::Point{x,y,z} = self {
            return (x,y,z);
        }
        panic!("Not a Point variant");
    }
}

// Option enum **Important**
// #[allow(dead_code)]
// enum Option<T>{
//     None,
//     Some(T),
// }

fn main() {
    let _primary = IpAdress::V4(127, 0, 0, 1);
    let _sec = IpAdress::V6(String::from("::1"));

    let a = Exmaples::Point { x: 10, y: 20, z: 15 };
    let (x,y,z) = a.dot();
    println!("the point is on x: {x}, y: {y}, z: {z}");

    let _number = Some(5);
    let _char = Some("sahil");
    let _null: Option<i32> = None;
}