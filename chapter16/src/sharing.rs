use std::sync::Mutex;

pub fn sample_mutx () {
    let x = Mutex::new(7);
    {
        let mut point =  x.lock().unwrap(); // MutecGaurd impliments the Deref and Drop trait
        *point = 13;
    }
    // The lock in droped
    println!("new x: {:?}", x);
}