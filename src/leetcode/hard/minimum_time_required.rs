pub fn minimum_time_required(mut jobs: Vec<i32>, k: i32) -> i32 {
    jobs.sort_by_key(|&x| std::cmp::Reverse(x));
    let mut workers = vec![0; k as usize];
    min_time_iter(&jobs, &mut workers, 0, 0, std::i32::MAX).unwrap()
}
/// # Arguments
/// * `jobs` - current slice of jobs, we will try to assign `jobs[0]` to some worker
/// * `workers` - mutable slice of each worker total job time
/// * `k` - count of currently assigned robots
/// * `max` - current maximum job time for a worker
/// * `best` - current best time found before
pub fn min_time_iter(jobs: &[i32], workers: &mut [i32], k: usize, max: i32, best: i32) -> Option<i32> {
    if jobs.is_empty() { return Some(max); }
    if max > best { return None; }
    let job = jobs[0];
    (0..(k + 1).min(workers.len())).rev().scan(best, |b, i| {
        workers[i] += job;
        let res = min_time_iter(&jobs[1..], workers, k.max(i + 1), workers[i].max(max), *b);
        workers[i] -= job;
        *b = res.filter(|r| r < b).unwrap_or(*b);
        Some(res)
    }).flatten().min()
}