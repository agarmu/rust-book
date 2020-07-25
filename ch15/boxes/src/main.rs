use List::*;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    let a = Cons(4, Box::new(Cons(3, Box::new(Cons(5, Box::new(Nil))))));
    println!("{:?}", a);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
