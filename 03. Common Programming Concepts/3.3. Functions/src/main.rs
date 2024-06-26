fn main() {
    println!("Hello, world!");

    another_function(3);
    print_labeled_measurement(1, 'h');

    // 구문과 표현식이다. 
    // 구문은 선언과 관련됐다. 구문은 ;를 붙혀야 한다.
    // 나는 표현식 = return 이고, 표현식에서는 ;을 붙히지 않아도 된다고 이해했다.
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    // 실제로 plus_one 함수를 보면 ;이 붙지 않은 모습을 볼 수 있다. 
    // ;를 붙히면, i32를 return하게 되지 않고, 구문으로 작동된다.  
    // 구문은 값을 평가하지 않기에 ()로 표현되는 유닛 타입으로 표현됩니다. 
    // 따라서 아무것도 반환되지 않아 함수가 정의된 내용과 상충하게 되어 에러가 발생됩니다.
    let x = plus_one(5);
    println!("The value of x is: {x}");
    
}

// return의 type을 적어주는 것이 포인트
fn plus_one(x: i32) -> i32 {
    x + 1
}


fn another_function(x:i32) {
    println!("Another function. parameter:{x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}