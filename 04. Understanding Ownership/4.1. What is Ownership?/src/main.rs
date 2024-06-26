fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()이 문자열에 리터럴을 추가합니다
    println!("{}", s); // 이 줄이 `hello, world!`를 출력합니다

    {
        let s = String::from("hello"); // s는 이 지점부터 유효합니다
        // s를 가지고 무언가 합니다
    }   // 이 스코프가 종료되었고, s는 더 이상 유효하지 않습니다. // drop()이라는 함수가 자동호출 됨. 
    
    let s = String::from("hello");  // s가 스코프 안으로 들어옵니다

    takes_ownership(s);             // s의 값이 함수로 이동됩니다...
                                    // ... 따라서 여기서는 더 이상 유효하지 않습니다

    let x = 5;                      // x가 스코프 안으로 들어옵니다

    makes_copy(x);                  // x가 함수로 이동될 것입니다만,
                                    // i32는 Copy이므로 앞으로 계속 x를
                                    // 사용해도 좋습니다

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // 이렇게 튜플 형식으로 기존의 값을 반환 받을 수 있긴 하지만 너무 귀찮다!! -> 참조 기능 사용!

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()은 String의 길이를 반환합니다

    (s, length)
}


fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어옵니다
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어나고 `drop`이 호출됩니다.
  // 메모리가 해제됩니다.

fn makes_copy(some_integer: i32) { // some_integer가 스코프 안으로 들어옵니다
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어납니다. 별다른 일이 발생하지 않습니다.
