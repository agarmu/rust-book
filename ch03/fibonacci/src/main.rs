use std::io::{stdin, stdout, Write};

fn fibonacci(i: u64) -> Result<u128,(u64,u128)> {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut tmp: u128;
    for r in 2..i+1 {
        tmp = b;
        b = match a.checked_add(b) {
            Some(n) => n,
            None => return Err((r,b))
        };
        a = tmp;
    }
    Ok(a)
}

fn input(prompt: String) -> Result<String, String> {
    print!("{}", prompt);
    match stdout().flush() {
        Ok(_) => (),
        Err(_) => return Err(String::from("Failed to flush.")),
    };
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(_) => return Err(String::from("Failed to read/store input.")),
    };
    Ok(input)
}

fn main() {
    loop {
        let inp = match input(String::from("> ")) {
            Ok(v) => v,
            Err(r) => {
                eprintln!("{}",r);
                continue;
            }
        };
        let inp = inp.as_str().trim();
        match inp {
            "q" | "quit" => break,
            _ => (),
        }
        let i: u64 = match inp.parse::<u64>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("{} is not a valid numeric input.", inp);
                continue;
            }
        };
        match fibonacci(i) {
            Ok(n) => {
                println!("Result : {}",n);
            },
            Err((maxindex, maxval)) => {
                eprintln!("Overflow error."); 
                eprintln!("Max: {}, which is the {}th fibonacci number.", maxval, maxindex);
            }
        };
    }
}