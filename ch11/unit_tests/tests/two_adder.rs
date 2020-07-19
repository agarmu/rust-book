use unit_tests::*;

#[test]
fn adds_two() {
    for r in 0..20000 {
        assert_eq!(r + 2, add_two(r))
    }
}
