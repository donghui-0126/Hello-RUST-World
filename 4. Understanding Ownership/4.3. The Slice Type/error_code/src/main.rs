fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // 에러! <- 이미 word가 s의 소유권을 가지고 있는데, 변화시키려해서 에러가 뜨게됨. 

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
