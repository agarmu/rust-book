fn main() {
    let v1: Vec<u128> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
    let total: u128 = v1.iter().sum();
    assert_eq!(6, total);
    println!("{:?}", v1.iter().map(|x| 2 * x).collect::<Vec<_>>());
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    println!("{:?}", shoes_in_my_size(shoes, 10));

    let c = Counter::new();
    let v: Vec<_> = c.collect();
    println!("{:?}", v);

    using_other_iterator_trait_methods()
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn using_other_iterator_trait_methods() {
    let sum: Vec<_> = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .collect();
    println!("{:?}", sum);
    let sum: u32 = sum.into_iter().sum();
    println!("{}", sum)
}
