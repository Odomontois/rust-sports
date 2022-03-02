use std::collections::BTreeSet;

pub fn max_task_assign(mut tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
    tasks.sort();
    let workers: BTreeSet<_> = workers.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
    let mut r = 0..tasks.len() + 1;
    let able = |t: usize| {
        let mut workers = workers.clone();
        let mut p = pills;
        for &req in tasks[..t].iter().rev() {
            if let Some(&w) = workers.range((req, 0)..).next() {
                workers.remove(&w);
            } else if let Some(&w) = workers.range((req - strength, 0)..).next().filter(|_| p > 0) {
                workers.remove(&w);
                p -= 1
            } else {
                return false;
            }
        }
        true
    };
    while r.len() > 1 {
        let m = (r.start + r.end) / 2;
        r = if able(m) { m..r.end } else { r.start..m };
    }
    r.start as i32
}
