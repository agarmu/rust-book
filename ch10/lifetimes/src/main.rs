fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Does not work, because x goes out of scope before its reference
    /*let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}",r);*/

    //Instead, use:
    let r;
    let x = 5;
    r = &x;
    println!("{}", r);

    //Longest string.
    let string1 = String::from("abcd");
    let string2 = "xyz";

    //should be abcd
    let res = longest(string1.as_str(), string2);
    println!("{}", res);
}
