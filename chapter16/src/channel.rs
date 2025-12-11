use std::{sync::mpsc, thread, time::Duration};

pub fn sample_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messege = "messege from the thread".to_string();
        tx.send(messege).unwrap();
    });

    println!("{}", rx.recv().unwrap());
}

pub fn channel_iter() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = vec!["take", "good", "care", "of", "the", "environment"];
        for i in msg {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for j in rx {
        println!("Messege recieved:\n{}", j);
    }
}
