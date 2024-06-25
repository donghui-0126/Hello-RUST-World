#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())  // unwrap_or_else로 Option을 처리함. 
                                                                // 함수를 사용할 수 있음. 
                                                                // Option으로 인수를 넘겨준다! => None이 가능함.
                                                                 
    }

    fn most_stocked(&self) -> ShirtColor { // 재고가 많은 옷을 넘겨줌
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


use std::thread;
use std::time::Duration;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    ////////////////////////////////////////////////////
    // let wait_time = expensive_closure(2);
    // println!("wait_time: {wait_time}");
    ////////////////////////////////////////////////////
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7); // list의 가변참조자가 borrows_mutably 클로저로 이동
    // println!("After calling closure: {:?}", list); // 따라서 list에서 사용이 불가능
    borrows_mutably(); // list의 가변참조자 반환

    println!("After calling closure: {:?}", list);
    ///////////////////////////////////////////////////
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list)) // 만약 새로운 스레드에서 클로저를 사용할 경우 move를 명시해줘야함
        .join()
        .unwrap();
}