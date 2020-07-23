pub fn find_wally<'a>(vec: &'a [&'a str]) -> Option<&'a &'a str> {
    vec.iter().find(|&&x| x == "wally")
}

pub fn find_wallys_position<'a>(vec: &'a [&'a str]) -> Option<usize> {
    vec.iter().position(|&x| x == "wally")
}

pub fn find_third_even(vec: &[i32]) -> Option<&i32> {
    vec.iter().filter(|&&x| x % 2 == 0).nth(2)
}

#[test]
fn test_ex4_find_wally() {
    let vec = vec!["dolly", "wally", "molly"];
    assert_eq!(find_wally(&vec), Some(&"wally"));

    let vec = vec!["cat", "pat"];
    assert_eq!(find_wally(&vec), None);
}

#[test]
fn test_ex4_find_wallys_position() {
    let vec = vec!["dolly", "wally", "molly"];
    assert_eq!(find_wallys_position(&vec), Some(1));

    let vec = vec!["dolly", "molly", "pat", "wally"];
    assert_eq!(find_wallys_position(&vec), Some(3));

    let vec = vec!["cat", "pat"];
    assert_eq!(find_wallys_position(&vec), None);
}

#[test]
fn test_ex4_find_third_even() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(find_third_even(&vec), Some(&6));

    let vec = vec![-4, -6, 0, 1, 2, 3, 4, 5];
    assert_eq!(find_third_even(&vec), Some(&0));

    let vec = vec![-2, -1];
    assert_eq!(find_third_even(&vec), None);
}
