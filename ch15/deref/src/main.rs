use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

// Deref Coercion
fn hello(name: &str) {
    println!("{}", name);
}
