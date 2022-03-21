pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
    batteries.sort_by_key(|&x| -x);
    let viable = |k: i64| -> bool {
        batteries
            .iter()
            .map(|&x| x as i64)
            .try_fold((n, k), |(computers, demand), charge| {
                if demand > charge {
                    Some((computers, demand - charge))
                } else if computers == 1 {
                    None
                } else {
                    Some((computers - 1, (k - charge + demand).max(demand)))
                }
            })
            .is_none()
    };
    let mut low = 0;
    let mut high = batteries.iter().map(|&x| x as i64).sum::<i64>() / n as i64 + 1;
    while high - low > 1 {
        let mid = (low + high + 1) / 2;
        if viable(mid) {
            low = mid
        } else {
            high = mid
        }
    }
    low
}

#[test]
fn test1() {
    assert_eq!(max_run_time(2, vec![3, 3, 3]), 4)
}

#[test]
fn test2() {
    assert_eq!(max_run_time(2, vec![1, 1, 1, 1]), 2)
}

#[test]
fn test3() {
    assert_eq!(max_run_time(3, vec![3, 4, 5, 3]), 5)
}
