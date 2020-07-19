fn main() {
    let mut v: Vec<i32> = Vec::new();
    //i32 - default integer type
    let r = vec![1, 2, 3];
    let q = &r[0];
    // push elements
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //Print result
    println!("{:?} {:?} {:?}", v, r, q);

    //Pop vals
    println!("{:?}", v.pop());
    println!("{:?}", v);
}
