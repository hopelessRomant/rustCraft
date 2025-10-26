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

fn main() {
    let _primary = IpAdress::V4(127, 0, 0, 1);
    let _sec = IpAdress::V6(String::from("::1"));
}