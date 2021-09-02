use std::cmp::Ordering::*;

struct Job {
    start: i32,
    end: i32,
    profit: i32,
}

pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mapper = |((start, end), profit)| Job { start, end, profit };
    let mut jobs: Vec<_> = start_time.into_iter().zip(end_time).zip(profit).map(mapper).collect();
    jobs.sort_by_key(|job| job.end);
    let mut res = vec![0];

    let step = |best: i32, current: &Job| {
        let compare = |job: &Job| {
            if job.end <= current.start {
                Less
            } else {
                Greater
            }
        };
        let i = jobs.binary_search_by(compare).unwrap_or_else(|i| i).min(res.len() - 1);
        let best = best.max(res[i] + current.profit);
        res.push(best);
        best
    };

    jobs.iter().fold(0, step)
}
