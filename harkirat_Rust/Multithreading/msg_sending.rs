use std::sync::mpsc; // multiple producer single consumer
use std::thread;

// take data from one thread and pass it to another thread

fn main(){
    let (tx,rx) = mpsc::channel(); // transmitter , receiver

    thread::spawn(move|| {
        let val = String::from("hello there");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got : {} ",received);
}