use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return;
    }
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Unable to read file.");
    println!("With Contents: {}", contents);
}
