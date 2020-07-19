use unit_tests::*;
mod common;
#[test]
fn adds_seventeen() {
    for r in 0..20000 {
        //internal_adder will not work b/c it is private
        assert_eq!(r + 17, external_adder(r, 17))
    }
}
