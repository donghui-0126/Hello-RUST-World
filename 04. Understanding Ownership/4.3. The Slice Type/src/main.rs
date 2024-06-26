fn main() {
    let mut s = String::from("hello world");

    let hello = &s[..5];
    {
        let world = &s[6..11];
        println!("{}", world);
    }
    // println!("{}", hello);    // println!을 해서 hello의 scope를 끝내버린다. 
    
    let mut_pointer = &mut s; // 여기서 가변 참조자를 만든다.
    // println!("{}", hello); <- 이렇게 쓰면 hello의 scope가 끝났기 때문에 오류가 뜬다.
    println!("{}", mut_pointer);

    let hello = &s[..5];
    println!("{}", hello);


    let slice = slice(&s[..]);
    println!("{}", slice);
    
    let slice = &slice[..];
}

fn slice(s:&str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}
