fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name)
                            .copied() // <- 굳이 왜 copied를 해야하는걸까? 
                            .unwrap_or(0); // <- get으로 원래는 Option을 받아오는데, unwrap_or로 만약 None값이면 0으로 리턴하도록 함. 



    let team_name_Red = String::from("Red");
    let score_Red = scores.get(&team_name_Red);    
    
    match score_Red { // <- 이렇게 match를 사용할 수도 있을 것 같다.
        Some(elem) => println!("{elem}"), 
        None => println!("0") 
    };

    for (key, value) in &scores {
        println!("{key}: {value}");
    };

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name과 field_value는 이 시점부터 유효하지 않습니다.
    // 사용을 시도해보고 무슨 컴파일러 에러가 발생하는 알아보세요! <-  value borrowed here after move 라는 에러가 뜬다.

    // entry 메소드를 사용해서 이런 기교도 가능하다.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // <- or_insert는 가변참조자 &mut V를 반환한다.
        *count += 1; // <- 즉, 가변참조자이기 때문에 오류가 나지 않는다.
    }

    println!("{:?}", map);
}