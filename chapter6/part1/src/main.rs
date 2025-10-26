#[allow(dead_code)]
enum IpAdress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let _primary = IpAdress::V4(127, 0, 0, 1);
    let _sec = IpAdress::V6(String::from("::1"));
}