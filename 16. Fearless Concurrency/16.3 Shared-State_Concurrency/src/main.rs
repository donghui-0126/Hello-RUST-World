// 메모리로 동시성을 하는 방법 // 하지말라는 것은 더욱 재밌는 법.
// 스마트 포인터 내용이 나와서 벽느껴짐... 

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5);
    println!("m = {:?}", m);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    ///////////////////////////////////////////
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}