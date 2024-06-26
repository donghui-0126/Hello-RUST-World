#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --생략--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn main() {
    let config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("Not u8!")
    }

    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}