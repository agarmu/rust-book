use super::*;

#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    println!("{}", &contents);
    let l = search(query, contents);
    println!("{:?}", l);
    assert_eq!(vec!["safe, fast, productive."], l)
}
