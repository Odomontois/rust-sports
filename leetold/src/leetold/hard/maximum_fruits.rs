pub fn max_total_fruits(fruits: Vec<impl AsRef<[i32]>>, start_pos: i32, k: i32) -> i32 {
    let fruits = fruits.iter().map(|x| x.as_ref());
    let left_to_right = fruits.clone().map(|v| [v[0], v[1]]);
    let right_to_left = fruits.rev().map(|v| [-v[0], v[1]]);
    back_and_forth(left_to_right, start_pos, k).max(back_and_forth(right_to_left, -start_pos, k))
}

pub fn back_and_forth(fruits: impl Iterator<Item = [i32; 2]> + Clone, pos: i32, k: i32) -> i32 {
    let mut sum = 0;
    let mut best = 0;
    let mut u = fruits.clone().peekable();
    for [p, c] in fruits.clone().take_while(|x| x[0] <= pos + k) {
        sum += c;
        if p < pos {
            continue;
        }
        while let Some(&[_, z]) = u.peek().filter(|x| pos - 2 * x[0] + p > k) {
            u.next();
            sum -= z;
        }
        best = best.max(sum)
    }
    best
}

#[test]
fn test1() {
    assert_eq!(9, max_total_fruits(vec![[2, 8], [6, 3], [8, 6]], 5, 4));
    assert_eq!(9, max_total_fruits(vec![vec![2, 8], vec![6, 3], vec![8, 6]], 5, 4));
    assert_eq!(9, max_total_fruits(vec![&[2, 8], &[6, 3], &[8, 6]], 5, 4));
}

#[test]
fn test2() {
    assert_eq!(
        14,
        max_total_fruits(vec![[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]], 5, 4)
    )
}
