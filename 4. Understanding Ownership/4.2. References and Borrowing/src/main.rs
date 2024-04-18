fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1); // 따란~ 이렇게 참조를 사용해서 함수가 직접 메모리에 접근할 수 있게 된다. 

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    println!("changed String: {}", s1);
    
    
    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    println!("{} and {}", r1, r2); // 당연하지만 놀랍게도 println!도 함수...
    // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다

    let r3 = &mut s; // 문제없음
    println!("{}", r3);


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}