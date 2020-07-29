use nqueens::run;
use std::env;
use std::process::exit;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: argument of board size must be specified.");
        exit(1);
    }
    let size: u8 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Only argument must be size of board. It must also be between 1-255");
        exit(1);
    });
    run(size);
}
