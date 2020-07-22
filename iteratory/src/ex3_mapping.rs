pub fn make_it_double(vec: &[i32]) -> Vec<i32> {
    todo!()
}

pub fn cap_to_ten(vec: &[i32]) -> Vec<i32> {
    todo!()
}

#[test]
fn test_ex3_make_it_double() {
    let vec = vec!(1, 2, 3, 4, 5);
    assert_eq!(make_it_double(&vec), vec!(2, 4, 6, 8, 10));

    let vec = vec!(-4, -6, 0, 1, 2, 3, 4, 5);
    assert_eq!(make_it_double(&vec), vec!(-8, -12, 0, 2, 4, 6, 8, 10));
}

#[test]
fn test_ex3_cap_to_ten() {
    let vec = vec!(1, 2, 13, 4, 15);
    assert_eq!(cap_to_ten(&vec), vec!(1, 2, 10, 4, 10));

    let vec = vec!(-4, -7, -6, 0, 100, 22, 3, 344, 5);
    assert_eq!(cap_to_ten(&vec), vec!(-4, -7, -6, 0, 10, 10, 3, 10, 5));
}
