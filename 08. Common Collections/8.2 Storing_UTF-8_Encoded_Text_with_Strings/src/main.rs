// 이부분이 참 묘하다. 왜냐하면 문자열을 포괄하는 타입이 두개다. 
// str과 String이 있는데, 두개가 묘하게 다르고 비슷하다..

//https://blog.cro.sh/posts/four-years-of-rust/#%EB%86%92%EC%9D%80-%EC%A7%84%EC%9E%85-%EC%9E%A5%EB%B2%BD 
// 위 블로그 설명을 잘 읽어보자

fn main() {
    let mut s = String::new(); // <- 얘는 String 타입..

    let data = "initial contents"; // <- 얘는 문자열 리터럴이여서 &str 타입..

    let s = data.to_string(); // <- 얘는 문자열 리터럴을 String으로 변환하는거
    // 이 메서드는 리터럴에서도 바로 작동합니다:
    let s = "initial contents".to_string();


    // String 타입은 이렇게도 사용가능
    let mut s = String::from("foo");
    s.push_str("bar");

}
