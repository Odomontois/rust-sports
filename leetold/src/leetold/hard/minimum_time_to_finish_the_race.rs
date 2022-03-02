pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
    let mut best = vec![];
    let mut check = |i: usize, x: i32| {
        if best.len() <= i {
            best.push(x)
        } else {
            best[i] = best[i].min(x)
        }
    };
    for tire in tires {
        if let &[f, r] = tire.as_slice() {
            check(0, f);
            let mut cur = f;
            let mut sum = f;
            for i in 1.. {
                if cur as i64 * (r as i64) >= change_time as i64 + f as i64 {
                    break;
                }
                cur *= r;
                sum += cur;
                check(i, sum);
            }
        }
    }
    let mut laps = vec![-change_time];
    for i in 0..num_laps as usize {
        let cur = (0..best.len().min(i + 1)).map(|k| laps[i - k] + best[k]).min();
        laps.push(cur.unwrap_or(0) + change_time);
    }
    println!("{laps:?}");
    laps[num_laps as usize]
}
