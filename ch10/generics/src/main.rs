fn largest_num(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// Instead of using specific functions, we can use a generic.

// Code below doesn't compile because of traits, but otherwise would be the best way to get things done.
/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/

// We can also have generic structs

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

//Method only for Point<f32>, will not work on Point<i32> or other.
impl Point<f32> {
    fn dist_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Multi-type Generics
#[derive(Debug, Copy, Clone)]
struct Mpt<T, U> {
    a: T,
    b: U,
}

impl<T, U> Mpt<T, U> {
    fn mixup<V, W>(self, other: Mpt<V, W>) -> Mpt<T, W> {
        Mpt {
            a: self.a,
            b: other.b,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("Largest num: {}", largest_num(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q', 'f', 't', 'p'];

    println!("Largest char: {}", largest_char(&char_list));
    let int_pt = Point { x: 1, y: 3 };
    let flt_pt = Point { x: 2.4, y: 3.7 };
    println!("{:?}, {:?}", int_pt, flt_pt);
    println!("({},{})", int_pt.x(), int_pt.y());
    println!("({},{})", flt_pt.x(), flt_pt.y());
    println!(
        "Dist from origin of ({},{}): {}",
        flt_pt.x(),
        flt_pt.y(),
        flt_pt.dist_origin()
    );

    //Does not compile b/c dist_origin() is only defined for Point<f32>, not Point<i32>
    //println!("Dist from origin of ({},{}): {}",int_pt.x(),int_pt.y(),int_pt.dist_origin());

    let p1 = Mpt { a: 5, b: "Hello" };
    let p2 = Mpt { a: 'r', b: false };
    println!("{:?}", p1.mixup(p2));
    println!("{:?}", p2.mixup(p1));
}
