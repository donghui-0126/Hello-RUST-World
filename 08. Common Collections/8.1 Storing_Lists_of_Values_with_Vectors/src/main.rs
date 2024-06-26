fn main() {
    let mut string_vec: Vec<String> = Vec::new();

    string_vec.push(String::from("111"));
    string_vec.push(String::from("222"));
    string_vec.push(String::from("333"));
    string_vec.push(String::from("444"));

    // 인덱스에 접근할 때는 웬만하면 &를 붙혀야한다.
    // String 타입은 Copy가 되지 않고 소유권을 이전해주어야 한다. 
    let third: &String = &string_vec[2]; 
    println!("The third element is {third}");

    

    let third: Option<&String> = string_vec.get(2); // 인덱스에 접근할 때는 &를 붙혀야한다.
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &string_vec {
        println!("{i}");
    }

    let mut int_v = vec![1, 2, 3, 4, 5];
    for i in &mut int_v {
        *i += 50; // 역참조
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let row_element: &SpreadsheetCell = &row[2]; // 이거도 Copy가 되지 않기 때문에 &를 붙혀줘야한다. 
    println!("{:?}",row_element)
}
