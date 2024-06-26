struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn bulid_user(email:String, username:String) -> User {
    User {
        active:true,
        username,
        email,
        sign_in_count:1
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("email@gmail.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{0}", user1.username);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    }; //user2를 생성한 이후에는 user1.username, user1.email 을 더 이상 사용할 수 없다. 
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}