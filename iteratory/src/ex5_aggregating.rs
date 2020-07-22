pub fn count<T>(vec: &[T]) -> usize {
    todo!()
}

pub fn concat(vec: &[char]) -> String {
    todo!()
}

pub fn are_all_negative(vec: &[i32]) -> bool {
    todo!()
}

pub fn contains_zero(vec: &[i32]) -> bool {
    todo!()
}

#[test]
fn test_ex5_count() {
    let vec = vec!("dolly", "wally", "molly");
    assert_eq!(count(&vec), 3);

    let vec = vec!(1, 2, 3, 4);
    assert_eq!(count(&vec), 4);
}

#[test]
fn test_ex5_concat() {
    let vec = vec!('p', 'a', 't');
    assert_eq!(concat(&vec), "pat");

    let vec = vec!('1', '2', '3', '4');
    assert_eq!(concat(&vec), "1234");
}

#[test]
fn test_ex5_are_all_negative() {
    let vec = vec!(-3, -7, 0, -6);
    assert_eq!(are_all_negative(&vec), false);

    let vec = vec!(-1, -2, -3, -4);
    assert_eq!(are_all_negative(&vec), true);
}

#[test]
fn test_ex5_contains_zero() {
    let vec = vec!(-3, -7, 0, -6);
    assert_eq!(contains_zero(&vec), true);

    let vec = vec!(-1, -2, -3, -4);
    assert_eq!(contains_zero(&vec), false);
}

