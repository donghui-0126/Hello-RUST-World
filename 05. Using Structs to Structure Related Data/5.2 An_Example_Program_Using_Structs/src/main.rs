struct Rectangle {
    width: u32,
    height: u32
}

// &를 붙히지 않으면 struct의 소유권이 함수로 넘어온다.
fn area(rectangle:&Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rectangle1 = Rectangle {
        width: 32,
        height: 64
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle1)
    );
}