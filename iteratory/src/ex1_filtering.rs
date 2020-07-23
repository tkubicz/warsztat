pub fn only_less_then_three(vec: &[i32]) -> Vec<i32> {
    vec.iter()

        .cloned()
        .collect::<Vec<_>>()
}

pub fn only_odd_numbers(vec: &[i32]) -> Vec<i32> {
    vec.iter()

        .cloned()
        .collect::<Vec<_>>()
}

#[test]
fn test_ex1_only_less_then_three() {
    let vec = vec!(1, 2, 3, 4, 5);
    assert_eq!(only_less_then_three(&vec), vec!(1, 2));

    let vec = vec!(-4, -6, 0, 1, 2, 3, 4, 5);
    assert_eq!(only_less_then_three(&vec), vec!(-4, -6, 0, 1, 2));
}

#[test]
fn test_ex1_only_odd_numbers() {
    let vec = vec!(1, 2, 3, 4, 5);
    assert_eq!(only_odd_numbers(&vec), vec!(1, 3, 5));

    let vec = vec!(-4, -7, -6, 0, 1, 2, 3, 4, 5);
    assert_eq!(only_odd_numbers(&vec), vec!(-7, 1, 3, 5));
}
