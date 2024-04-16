fn main() {
    // 간단한 if문 예시
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // let true = 1; // RUST에서는 자동으로 int->bool 변환을 하지 않는다.
    let _bool_variable:bool = true;
    if true {
        println!("True!");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 변수 선언시에 if문을 사용할 수 있다. 
    // 하지만 if else로 선언되는 변수의 type가 같아야한다. <- RUST는 컴파일시에 Type가 정해져야하기 때문이다. 
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // 간단한 loop 예시
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");


    // 간단한 while 예시
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");


    // array 타입에 대한 간단한 예시 
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for문을 사용해서 아래와 같이 코드를 수정할 수도 있다.
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // python으로 치면 range 함수도 있다.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
