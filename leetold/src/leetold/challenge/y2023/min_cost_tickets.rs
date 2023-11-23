pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let mut res = vec![i32::MAX; days.len()];
    for (j, &d) in days.iter().enumerate() {
        let prev = if j == 0 { 0 } else { res[j - 1] };
        for (&cost, time) in costs.iter().zip([1, 7, 30]) {
            let l = days[j..].iter().take_while(|&&d1| d1 - d < time).count();
            (j..j + l).for_each(|i| res[i] = res[i].min(prev + cost));
        }
    }
    res.last().copied().unwrap_or(0)
}
