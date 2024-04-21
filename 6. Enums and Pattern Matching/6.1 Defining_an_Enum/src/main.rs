enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 메서드 본문이 여기 정의될 것입니다
    }
}

let m = Message::Write(String::from("hello"));
m.call();

enum Option<T> {
    None,
    Some(T),
}


fn main() {
struct Ipv4Addr {
    // --생략--
}

struct Ipv6Addr {
    // --생략--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;

}