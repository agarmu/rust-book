trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        //Default
        format!("(Read more from {}..)", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    _content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        (*self.author).to_string()
    }
}
#[derive(Debug)]
struct Tweet {
    username: String,
    _content: String,
    _reply: bool,
    _retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    //No need to implement summarize(), since it has a default.
}
//trait bounds
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
//impl syntax
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//where syntax
fn _some_func<T, U>(_t: &T, _u: &U) -> i32
where
    T: Summary + Clone,
    U: Clone + std::fmt::Debug,
{
    0
}

//return Trait - does not compile - see ch17
/*
fn returns_summarizable(switch: bool) -> impl Summary{
    if switch {
        return NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };
    } else {
        return Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };
    }
}
*/

// Conditionally Implement Methods using Trait Bounds.

use std::fmt::Display;
#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Debug> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        _content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    notify(&article);
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        _content: String::from("of course, as you probably already know, people"),
        _reply: false,
        _retweet: false,
    };
    let tweetb = Tweet {
        username: String::from("horse_ebooks"),
        _content: String::from("are very very stupid."),
        _reply: false,
        _retweet: false,
    };
    notify(&tweet);

    let number_list = vec![34, 50, 25, 100, 65];

    println!("Largest num: {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q', 'f', 't', 'p'];

    println!("Largest char: {}", largest(&char_list));

    let pair_a = Pair::new(5, 6);
    let pair_b = Pair::new(tweet, tweetb);
    println!("pair_a:{:?}", &pair_a);
    println!("pair_b:{:?}", &pair_b);

    //Works
    pair_a.cmp_display();

    //Does not work
    //pair_b.cmp_display();
}
