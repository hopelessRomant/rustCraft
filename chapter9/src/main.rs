#[allow(dead_code)]
mod result;

fn main() {
    // result::intro();
    // result::create();
    match result::read() {
        Ok(data) => print!("read data is:\n{}", data),
        Err(e) => panic!("Shit happened: {:#?}", e),
    };
}
