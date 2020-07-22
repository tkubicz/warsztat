// Hint:
// Closure passed to iterator methods takes a reference,
// and many iterators iterate over references,
// this leads to a possibly confusing situation,
// where the type of the closure is a double reference.
//
// To dereference use asterisk. Thus double dereference will be: **x

pub fn at_most_three(vec: &[i32]) -> Vec<i32> {
    vec.iter()

        .cloned()
        .collect::<Vec<_>>()
}

pub fn up_to_ten(vec: &[i32]) -> Vec<i32> {
    vec.iter()

        .cloned()
        .collect::<Vec<_>>()
}

pub fn without_first_two(vec: &[i32]) -> Vec<i32> {
    vec.iter()

        .cloned()
        .collect::<Vec<_>>()
}

pub fn not_until_five(vec: &[i32]) -> Vec<i32> {
    vec.iter()

        .cloned()
        .collect::<Vec<_>>()
}

#[test]
fn test_ex2_at_most_3() {
    let vec = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

    assert_eq!(at_most_three(&vec), vec!(1, 2, 3));
}

#[test]
fn test_ex2_up_to_ten() {
    let vec = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    assert_eq!(up_to_ten(&vec), vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10));

    let vec = vec!(1, 3, 6, 10, 15, 5, 20);
    assert_eq!(up_to_ten(&vec), vec!(1, 3, 6, 10));
}

#[test]
fn test_ex2_without_first_two() {
    let vec = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    assert_eq!(without_first_two(&vec), vec!(3, 4, 5, 6, 7, 8, 9, 10, 11, 12));

    let vec = vec!(1, 3, 6, 10, 15, 20);
    assert_eq!(without_first_two(&vec), vec!(6, 10, 15, 20));
}

#[test]
fn test_ex2_not_until_five() {
    let vec = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    assert_eq!(not_until_five(&vec), vec!(5, 6, 7, 8, 9, 10, 11, 12));

    let vec = vec!(1, 3, 6, 10, 15, 20, 5, 3, 9);
    assert_eq!(not_until_five(&vec), vec!(5, 3, 9));
}
