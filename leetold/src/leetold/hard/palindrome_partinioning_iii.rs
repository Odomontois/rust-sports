pub fn palindrome_partition(s: String, k: i32) -> i32 {
    let mc_calc = |i| (i + 1..=s.len()).map(|j| min_chars(&s[i..j])).collect();
    let mc: Vec<Vec<_>> = (0..s.len()).map(mc_calc).collect();
    let step_calc = |i: usize, u, cur: &[usize]| (0..s.len() - i - u).map(|l| mc[i][l] + cur[i + l + 1]).min();
    let step = |cur: Vec<_>, u| (0..s.len() - u).filter_map(|i| step_calc(i, u, &cur)).collect();
    (1..k as usize).fold((0..s.len()).filter_map(|i| mc[i].last()).cloned().collect(), step)[0] as i32
}

fn min_chars(s: &str) -> usize {
    s.chars()
        .zip(s.chars().rev())
        .take(s.len() / 2)
        .filter(|&(x, y)| x != y)
        .count()
}

#[test]
fn test() {
    assert_eq!(palindrome_partition("abc".to_string(), 2), 1);
    assert_eq!(palindrome_partition("aabbc".to_string(), 3), 0);
    assert_eq!(palindrome_partition("leetcode".to_string(), 8), 0);
    assert_eq!(palindrome_partition("leetcode".to_string(), 7), 0);
    assert_eq!(palindrome_partition("leetcode".to_string(), 6), 1);
}
