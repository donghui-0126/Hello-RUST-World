use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    // main 함수에서 인수 parsing 하지 않고 함수에서 처리
    // -> 깔끔해짐 
    let config = Config::build(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}


struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl Config<'_> {
    fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}