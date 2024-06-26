use std::sync::mpsc;
use std::thread;
use core::time::Duration;

fn main() {
    // transmitter
    // receiver
    let (tx, rx) = mpsc::channel(); // tx1 = tx.clone() 방법으로 여러 송신자를 만들 수 있음.

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    /////////////////////////////////////////
    // let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
