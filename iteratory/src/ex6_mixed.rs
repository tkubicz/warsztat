pub fn triples_of_odds(vec: &[i32]) -> Vec<i32> {
    vec.iter()
        .filter(|&&x| x % 2 != 0)
        .map(|&x| x * 3)
        .collect()
}

#[test]
fn test_ex6_triples_of_odds() {
    let vec = vec![1, 2, 3, 4];
    assert_eq!(triples_of_odds(&vec), vec!(3, 9));

    let vec = vec![-1, -2, -3, 4, 5];
    assert_eq!(triples_of_odds(&vec), vec!(-3, -9, 15));
}
