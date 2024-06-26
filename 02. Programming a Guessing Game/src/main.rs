use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let parse_result: Result<u32, _> = guess.trim().parse();

        // println!("\tParsing result: {:?}", parse_result); // Result 열거형 출력

        let guess: u32 = match parse_result {
            Ok(num) => {
                num
            }
            Err(_) => {
                println!("Invalid input, please enter a valid number."); // 오류 발생 시 메시지 출력
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
