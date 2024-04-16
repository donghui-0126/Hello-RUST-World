const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // mut을 붙히지 않으면 변경이 불가하다.
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // const 기본으로 불변형임. 
    // const는 type을 지정해줘야한다. 
    println!("Contant: {THREE_HOURS_IN_SECONDS}");


    // shadowing
    // shadowing은 덮어쓰는 개념
    // 변수의 타입을 자연스럽게 바꿀 수 있음
    let spaces = "   ";             // String 타입
    println!("space: |{spaces}|");
    let spaces = spaces.len();      // Int 타입
    println!("space length: {spaces}");
    // 아래와 같이 코드를 짜면 타입이 바뀌기 때문에 에러가 뜸
    // let mut spaces = "   ";
    // spaces = spaces.len();


}