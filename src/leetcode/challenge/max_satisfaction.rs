pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
    satisfaction.sort_unstable();
    let accum = |acc: &mut i32, &x| {
        *acc += x;
        Some(*acc)
    };
    let sums: Vec<_> = satisfaction.iter().rev().scan(0, accum).collect();
    let all: i32 = satisfaction.iter().enumerate().map(|(i, &x)| x * (i as i32 + 1)).sum();
    sums.iter().rev().fold((all, 0), |(p, b), &s| (p - s, b.max(p - s))).1
}

#[test]
fn check1() {
    assert_eq!(14, max_satisfaction(vec![-1, -8, 0, 5, -9]));
}
