use std::collections::hash_map::*;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
/*fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}*/

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    calculation: T,
    value: HashMap<U, V>,
}
impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
    V: std::clone::Clone,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> V {
        match self.value.entry(arg.clone()) {
            Entry::Occupied(o) => (*o.get()).clone(),
            Entry::Vacant(v) => (v.insert((self.calculation)(arg))).clone(),
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
