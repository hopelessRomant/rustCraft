use std::sync::{Mutex, Arc};
use std::thread;

pub fn sample_mutx () {
    let x = Mutex::new(7); // the Mutex remains in place and .lock() method returns the mutable pointer (MutexGaurd) to it.
    {
        let mut point =  x.lock().unwrap(); // MutecGaurd impliments the Deref and Drop trait
        *point = 13;
    }
    // The lock is droped
    println!("new x: {:?}", *x.lock().unwrap());
}

pub fn arc() {
    let sample = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&sample);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    };

    for i in handles {
        i.join().unwrap();
    };

    println!("counter result: {}", *sample.lock().unwrap())
}
