fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();    // 기묘한 반복자의 세계..
                                                            // 반복자를 소비하기 위해 collect()로 묶어줌 
    assert_eq!(v2, vec![2, 3, 4]);
}