use std::{sync::mpsc, thread};

pub fn sample_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messege = "messege from the thread".to_string();
        tx.send(messege).unwrap();
    });

    println!("{}", rx.recv().unwrap());
}