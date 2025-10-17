// rust has 3 loops 
// * loop
// * for
// * while 

// passing result out of a loop
// fn main() {
//     let mut counter = 0;
//     let number = loop {
//         counter += 1;
//         if counter == 5 {
//             break counter*2; // return value 10
//         }
//     };
//     println!("value of number is : {number}")
// }

// loop labeling
fn main () {
    let mut count = 0;
    'counter: loop{
        println!("count : {count}");
        let mut life = 1;
        loop {
            println!("life : {life}");
            if life == 0 {
                break;
            }
            if count == 2 {
                break 'counter;
            }
            life -=1;
        }
        count+=1
    }
    println!("final count: {count}");
}
