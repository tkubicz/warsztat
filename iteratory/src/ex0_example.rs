pub fn just_iterate(vec: &[i32]) -> Vec<i32> {
    vec.iter()
        .cloned()
        .collect::<Vec<_>>()
}

#[test]
fn test_ex0_just_iterate() {
    let vec = vec!(1, 2, 3);

    assert_eq!(just_iterate(&vec), vec);
}
