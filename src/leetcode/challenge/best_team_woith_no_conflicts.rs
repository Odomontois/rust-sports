pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut best = vec![(0, 0)];
    let mut pairs: Vec<_> = scores.into_iter().zip(ages).collect();
    pairs.sort();
    for (score, age) in pairs {
        let mut cur = best
            .iter()
            .filter_map(|&(max_age, total)| (max_age <= age).then(|| total + score))
            .max()
            .unwrap_or(0);
        best.retain(|&(max_age, total)| max_age < age || total > cur);
        best.push((age, cur));
    }
    best.iter().map(|p| p.1).max().unwrap_or(0)
}
