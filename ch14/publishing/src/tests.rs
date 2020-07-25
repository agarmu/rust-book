use super::*;

/// This test ensures that the add one function is working as desired.
#[test]
fn it_works() {
    for i in 0..1000000 {
        assert_eq!(i + 1, add_one(i))
    }
}
