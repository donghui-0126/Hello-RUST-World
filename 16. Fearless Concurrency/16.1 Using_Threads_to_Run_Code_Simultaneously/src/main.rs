use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }); // 생성된 스레드는 메인 스레드가 실행할 때까지만 실행됨.

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // 이 코드를 통해서 스레드의 완결성 보장 가능

    ///////////////////////////////////////////////////////////////////
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // move를 넘겨줘야만 클로저를 사용할 수 있음. 
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    
}