fn main() {
    let x = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let y = [
        "twelve drummers drumming",
        "eleven pipers piping",
        "ten lords a leaping",
        "nine ladies dancing",
        "eight maids a milking",
        "seven swans a swimming",
        "six geese a laying",
        "five gold rings",
        "four calling birds",
        "three french hens",
        "two turtle doves",
        "a partridge in a pear tree",
    ];
    for (i, a) in x.iter().enumerate() {
        println!("On the {} day of Christmas, my true love gave to me:", a);
        if i == 0 {
            println!("\t{}.", y[11]);
        } else {
            for (j, b) in y[0..i + 1].iter().enumerate() {
                if j == i {
                    println!("\tand a {}.", b);
                } else {
                    println!("\t{},", b);
                }
            }
        }
    }
}
