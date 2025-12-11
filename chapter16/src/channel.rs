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

pub fn mpsc() {
    let (tx0, rx) = mpsc::channel();
    let tx1 = tx0.clone();

    thread::spawn(move || {
        let msg0 = vec!["Hi", "are"];
        for i in msg0 {
            tx0.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        let msg1 = vec!["how", "you"];
        for j in msg1 {
            tx1.send(j).unwrap();
        }
    });

    for msg in rx {
        println!("{}", msg);
    }
}
