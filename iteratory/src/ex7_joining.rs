pub fn pair_people<'a>(
    vec1: &'a [&'a str],
    vec2: &'a [&'a str],
) -> Vec<(&'a &'a str, &'a &'a str)> {
    vec1.iter().zip(vec2).collect()
}

pub fn sum_first_ten_elements(vec1: &[i32], vec2: &[i32]) -> i32 {
    vec![vec1, vec2].into_iter().flatten().take(10).sum()
}

pub fn sum_matrix(matrix: Vec<Vec<i32>>) -> i32 {
    matrix.into_iter().flatten().sum()
}

// For every element count its distance from its position in vector
pub fn distance_from_position(vec: &[i32]) -> Vec<i32> {
    vec.iter()
        .enumerate()
        .map(|(i, &val)| (val - i as i32).abs())
        .collect()
}

#[test]
fn test_ex7_pair_people() {
    let vec1 = vec!["pat", "mat"];
    let vec2 = vec!["molly", "dolly"];
    assert_eq!(
        pair_people(&vec1, &vec2),
        vec!((&"pat", &"molly"), (&"mat", &"dolly"),)
    );
}

#[test]
fn test_ex7_sum_first_ten_elements() {
    let vec1 = vec![1, 1, 1, 1];
    let vec2 = vec![1, 1, 1, 1, 1, 1, 1];
    assert_eq!(sum_first_ten_elements(&vec1, &vec2), 10);

    let vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![6, 7, 8, 9, 10, 11, 12];
    assert_eq!(sum_first_ten_elements(&vec1, &vec2), 61);
}

#[test]
fn test_ex7_sum_matrix() {
    let matrix = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

    assert_eq!(sum_matrix(matrix), 9);

    let matrix = vec![vec![1, 1, 1], vec![1, 2, 1], vec![3, 1, 1]];

    assert_eq!(sum_matrix(matrix), 12);
}

#[test]
fn test_ex7_distance_from_position() {
    let vec1 = vec![0, 1, 2, 3];
    assert_eq!(distance_from_position(&vec1), vec!(0, 0, 0, 0));

    let vec2 = vec![6, 4, 1, 0];
    assert_eq!(distance_from_position(&vec2), vec!(6, 3, 1, 3));
}
