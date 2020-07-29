use super::*;
#[test]
fn check_works() {
    let a: Vec<(Vec<isize>, bool)> = vec![
        (vec![1, 2, 3, 4, 5], false),
        (vec![1, 20, 30, 40, 50], true),
        (vec![1, 3, 4, 2, 5], false),
        (vec![1, 3, 0, 2], true),
    ];
    for r in a {
        assert_eq!(check(&r.0), r.1);
    }
}
