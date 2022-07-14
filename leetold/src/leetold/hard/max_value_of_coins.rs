pub fn max_value_of_coins<I: AsRef<[i32]>>(piles: Vec<I>, k: i32) -> i32 {
    piles.iter().fold(vec![0], |p, v| max_vals(p, v.as_ref(), k as usize))[k as usize]
}

fn max_vals(prev: Vec<i32>, pile: &[i32], max: usize) -> Vec<i32> {
    let n = max.min(prev.len() + pile.len() - 1);
    let pile_sums: Vec<_> = pile
        .iter()
        .scan(0, |s, x| {
            *s += x;
            Some(*s)
        })
        .collect();
    let calc = |i: usize| {
        let from = 0.max(i - pile.len().min(i));
        let until = prev.len().min(i);
        let sums = (from..until).map(|j| prev[j] + pile_sums[i - j - 1]);
        sums.chain(prev.get(i).copied()).max().unwrap_or(0)
    };
    (0..=n).map(calc).collect()
}

#[test]
fn test1() {
    assert_eq!(101, max_value_of_coins(vec![&[1, 100, 3], &[7, 8, 9]], 2))
}

#[test]
fn test2() {
    assert_eq!(
        706,
        max_value_of_coins(
            vec![
                &[100] as &[i32],
                &[100],
                &[100],
                &[100],
                &[100],
                &[100],
                &[1, 1, 1, 1, 1, 1, 700]
            ],
            7
        )
    )
}
