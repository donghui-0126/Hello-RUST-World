fn main() {
    // RUST의 Type은 c++과 거의 흡사한 것 같다. 
    // RUST의 Char는 더 많은 기호를 표기할 수 있다고 한다. 
    let c = 'z';
    let z: char = 'ℤ'; // 명시적인 타입 어노테이션
    let heart_eyed_cat = '😻';  

    // RUST에는 tuple이라는 타입이 있다. 
    // 이 타입은 서로 다른 자료형을 보관할 수 있다. 
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1); // 이렇게 선언할 수도 있다. 
    let (x, y, z) = tup; // 특정 값에 접근하려면 이렇게 해체하면 된다. 
    println!("The value of y is: {y}");
    
    // tup.index의 방식으로 인덱스로도 접근할 수 있다. 
    let tup_index_0 = tup.0;
    let tup_index_1 = tup.1;
    let tup_index_2 = tup.2;
    
    // 아래와 같이 해당되지 않는 인덱스에 접근하면 어떻게 될까
    // let tup_index_error = tup.3;
    // no field `3` on type `({integer}, {float}, {integer})` 이런 에러가 뜬다. 

    // 배열타입은 길이가 불변한다. 
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]으로 할당된다. 
    // 당연하지만 아래와 같이 index로 접근 가능하다.
    let first = a[0];
    let second = a[1];

    // 아래는 에러코드
    // c++과 다르게 메모리에 접근하지 않고 적절한 에러를 출력한다. 
    // let error = a[6] 
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


}
