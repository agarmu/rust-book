use colored::*;
use regex::Regex;
use std::io::{stdin, stdout, Write};
//[dependencies]
//regex = "1"
//colored = "1.9"

fn main() {
    let re =
        Regex::new(r#"(?x)(?P<num>(-?)([0-9]+)(\.)?([0-9]+)?)(?P<ft>(?P<frm>C|K|F)(?P<to>C|K|F))"#)
            .unwrap();
    help_msg();
    loop {
        let inp = match input(String::from("> ")) {
            Ok(s) => s,
            Err(r) => {
                eprintln!("{}. Please try again", r);
                continue;
            }
        };
        let inp = match inp.trim().parse::<String>() {
            Ok(s) => s,
            Err(_) => {
                println!("Internal error.");
                continue;
            }
        };
        match inp.as_str() {
            "q" | "quit" | "exit" => {
                println!(
                    "{}",
                    "Thanks for using this calculator."
                        .bright_green()
                        .bold()
                        .underline()
                );
                break;
            }
            "help" => {
                help_msg();
                continue;
            }
            "" => continue,
            _ => (),
        };
        let caps = match re.captures(inp.as_str()) {
            Some(v) => v,
            None => {
                eprintln!(
                    "{}",
                    "Invalid input. Type 'help' for help about input, or 'exit' to exit."
                        .bright_red()
                        .bold()
                );
                continue;
            }
        };

        let val: f64 = caps["num"].parse().unwrap();
        let ans = match &caps["ft"] {
            "FC" => f_to_c(val),
            "CF" => c_to_f(val),
            "CK" => c_to_k(val),
            "KC" => k_to_c(val),
            "KF" => k_to_f(val),
            "FK" => f_to_k(val),
            "KK" | "CC" | "FF" => val,
            _ => {
                eprintln!(
                    "{}",
                    "Invalid input. Type 'help' for help about input, or 'exit' to exit."
                        .bright_red()
                        .bold()
                );
                continue;
            }
        };
        println!(
            "{}",
            format!(
                "{:.2} deg {} = {:.2} deg {}",
                val, &caps["frm"], ans, &caps["to"]
            )
            .bright_green()
            .bold()
            .underline()
        );
    }
}

fn help_msg() {
    println!("{}", "Welcome to the temperature converter.".blue());
    println!(
        "{}",
        "Enter your temperature as a floating point number, and then two of these letters: ".blue()
    );
    println!("{}", "\tC : Celsius".yellow().bold());
    println!("{}", "\tF : Fahrenheit".yellow().bold());
    println!("{}", "\tK : Kelvin".yellow().bold());
    println!("{}","where the first is the one you wish to covert from and the second is the one to convert to.".blue());
    println!(
        "{}",
        "For example : 32FC converts 32 deg Fahrenheit to Celsius.".blue()
    );
    println!(
        "{}",
        "You can quit by typing 'quit', 'exit',or 'q'; or view this message again by typing 'help'"
            .blue()
    );
}

fn input(prompt: String) -> Result<String, String> {
    print!("{}", prompt);
    match stdout().flush() {
        Ok(_) => (),
        Err(_) => return Err(String::from("Failed to read input.")),
    };
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(_) => return Err(String::from("Failed to store input.")),
    };
    Ok(input)
}
fn f_to_c(f: f64) -> f64 {
    (f - 32.) * 5. / 9.
}

fn c_to_f(c: f64) -> f64 {
    (c * 9. / 5.) + 32.
}
fn c_to_k(c: f64) -> f64 {
    c + 273.15
}
fn k_to_c(k: f64) -> f64 {
    k - 273.15
}
fn f_to_k(f: f64) -> f64 {
    ((f - 32.) * 5. / 9.) + 273.15
}
fn k_to_f(k: f64) -> f64 {
    ((k - 273.15) * 9. / 5.) + 32.
}
