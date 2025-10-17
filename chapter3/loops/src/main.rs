// rust has 3 loops 
// * loop
// * for
// * while 

//passing result out of a loop
fn main() {
    let mut counter = 0;
    let number = loop {
        counter += 1;
        if counter == 5 {
            break counter*2; // return value 10
        }
    };
    println!("{number}")
}

