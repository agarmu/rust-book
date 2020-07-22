use std::env::var;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
#[cfg(test)]
mod tests;

pub struct Config {
    query: String,
    filename: PathBuf,
    case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not receive query"),
        };
        if query == "-h" || query == "--help" {
            println!("Usage:");
            println!("\tminigrep [query] [file path]");
            std::process::exit(0);
        }
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let filename = match fs::canonicalize(filename) {
            Ok(v) => v,
            Err(_) => return Err("Error parsing filename"),
        };
        let case_sensitive = var("CASE_INSENSITIVE").is_err();
        Ok(Self {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
