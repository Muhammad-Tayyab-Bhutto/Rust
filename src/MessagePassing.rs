use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);  // Clone tx for another thread
    let tx2 = mpsc::Sender::clone(&tx);
    let tx3 = mpsc::Sender::clone(&tx);
    // First thread
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // Second thread
    thread::spawn(move || {
        let val = String::from("from");
        tx1.send(val).unwrap();
    });

    // Third thread
    thread::spawn(move || {
        let val = String::from("the");
        tx2.send(val).unwrap();
    });

    // Fourth thread
    thread::spawn(move || {
        let val = String::from("thread");
        tx3.send(val).unwrap();
    });

    // Receiving and printing messages  
    for received in rx {
        println!("Got: {}", received);
    }
}
