pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
    let mut target: Vec<_> = target.into_iter().enumerate().collect();
    target.sort_by_key(|(_, x)| *x);
    let index = |x| target.binary_search_by_key(x, |&(_, y)| y).ok().map(|i| target[i].0);
    let mut tails = Vec::new();
    for x in arr.iter().filter_map(index) {
        if let Err(i) = tails.binary_search(&x) {
            if i == tails.len() {
                tails.push(x)
            } else {
                tails[i] = x
            }
        }
    }
    (target.len() - tails.len()) as i32
}

#[test]
fn check() {
    assert_eq!(min_operations(vec![5, 1, 3], vec![9, 4, 2, 3, 4]), 2);
    assert_eq!(min_operations(vec![6, 4, 8, 1, 3, 2], vec![4, 7, 6, 2, 3, 8, 6, 1]), 3);
}
