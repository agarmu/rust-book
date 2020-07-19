use super::*;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn adds_seventeen() {
    for r in 0..20000 {
        //internal_adder will not work b/c it is private
        assert_eq!(r + 17, internal_adder(r, 17))
    }
}
