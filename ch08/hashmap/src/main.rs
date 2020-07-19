use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<Result<Option<String>, Option<i32>>, i32> = HashMap::new();
    scores.insert(Ok(Some(String::from("Hi"))), 2i32);
    scores.insert(Err(None), 3i32);
    println!("{:#?}", scores);
}
