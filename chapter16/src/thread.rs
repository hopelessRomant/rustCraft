use std::thread;
use std::time::Duration;

pub fn thread_test () {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread count: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        print!("main count: {i} ");
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}

pub fn move_ownership() {
    let v = vec![2,3,5,7];

    // we use the move key word to move the ownership of values into the closure environment.
    let handle = thread::spawn(move || {
        println!("this value is now owned by the thread:\n {:?}", v)
    });

    handle.join().unwrap();
}
