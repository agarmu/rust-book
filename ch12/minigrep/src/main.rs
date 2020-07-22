use minigrep::Config;
use std::env::args;
use std::process::exit;

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments: {}", err);
        exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        exit(1);
    };
}
