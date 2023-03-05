use leet_code_rust_lang::problems::_0001_two_sum::Solution;

#[test]
fn example_1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn example_2() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
fn example_3() {
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
