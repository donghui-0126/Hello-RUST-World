fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &r1;

    println!("{}, {}", r1, r2);
}

fn dangle() -> &String { // dangle은 String의 참조자를 반환합니다

    let s = String::from("hello"); // s는 새로운 String입니다

    &s // String s의 참조자를 반환합니다
} // 여기서 s는 스코프 밖으로 벗어나고 버려집니다. 해당 메모리는 해제됩니다.
  // 위험합니다!