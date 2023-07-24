pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable_by_key(|v| v[0]);
    let f = |(k, e), v: &Vec<i32>| if v[0] < e { (k + 1, e.min(v[1])) } else { (k, v[1]) };
    intervals.iter().fold((0, intervals[0][0] - 1), f).0
}
