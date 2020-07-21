use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Self { query, filename })
    }
    pub fn query(&self) -> &String {
        &self.query
    }
    pub fn file(&self) -> &String {
        &self.filename
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}
