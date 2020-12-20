use std::collections::HashMap;
fn main() {
    println!("{:#?}", getvals(&mut vec![1, 2, 3, 3, 2]));
}

fn get_vals(vals: &mut Vec<i32>) -> (f32, f32, Vec<i32>) {
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let size = vals.len();
    // Get Mean
    let mean: f32 = (vals.iter().sum::<i32>() as f32) / (size as f32);

    //Get Median
    let median: f32;
    if size % 2 == 0 {
        median = ((vals[size / 2] + vals[size / 2 + 1]) as f32) / 2.;
    } else {
        median = vals[(size + 1) / 2] as f32;
    }
    let mut map = HashMap::new();

    for val in vals {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }

    //Get Mode using HashMap
    let mut mode: Vec<i32> = Vec::new();
    let mut count: i32 = 0;
    for (k, v) in map.iter() {
        if v > &count {
            count = *v;
            mode = vec![**k]
        } else if v == &count {
            mode.push(**k);
        }
    }
    (mean, median, mode)
}
