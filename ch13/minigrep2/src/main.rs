use minigrep2 as minigrep;
use std::env::args;
use std::process::exit;

fn main() {
    let config = minigrep::Config::new(args()).unwrap_or_else(|err| {
        eprintln!("Problem Parsing Arguments: {}", err);
        exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        exit(1);
    };
}
