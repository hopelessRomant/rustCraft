use std::thread;
use std::time::Duration;

pub fn thread_test () {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread count: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        print!("main count: {i} ");
        thread::sleep(Duration::from_millis(1));
    }

   
}


