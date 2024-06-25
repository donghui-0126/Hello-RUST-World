use std::env;
use std::process;   
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // main 함수에서 인수 parsing 하지 않고 함수에서 처리
    // -> 깔끔해짐 
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}