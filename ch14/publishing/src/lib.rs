//! This crate is a basic test crate for cargo documentation and
//! crates.io features.

#[cfg(test)]
pub mod tests;

/// Adds one to the provided number
/// # Examples
///
/// ```
/// let arg = 5;
///
/// // Should be 5+1 = 5
/// let answer = publishing::add_one(arg);
/// // Asserting this to be true
/// assert_eq!(6, answer);
/// # //This is invisible :) :)
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}
