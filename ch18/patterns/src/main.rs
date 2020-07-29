fn main() {
    example(true, "if let", || {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    });
    example(true, "while let", || {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    });
    example(true, "for loops", || {
        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    });
    example(true, "parameter patterns", || {
        print!("{:?}", pattern_func(&(5, 7)))
    });
}

fn example<T: Fn() -> ()>(enable: bool, name: &str, a: T) {
    if enable {
        println!("\n\nExample: {}", name);
        a();
        print!("\n");
        for _ in 0..25 {
            print!("-");
        }
        print!("\n");
    }
}

fn pattern_func(&(x, y): &(i32, i32)) -> bool {
    x == y
}
