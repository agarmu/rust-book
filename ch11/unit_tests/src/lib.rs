//#[cfg(test)] only compiles mod tests when cargo test is run, rather than build.
//It is not necessary for integration tests, only unit tests.

#[cfg(test)]
//Unit tests
mod tests;

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn external_adder(a: i32, b: i32) -> i32 {
    internal_adder(a, b)
}
