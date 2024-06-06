pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    use std::collections::HashMap;
    let n = group_size as usize;
    if hand.len() % n != 0 {
        return false;
    }
    let mut counts = HashMap::new();
    hand.into_iter().for_each(|x| *counts.entry(x).or_insert(0) += 1);
    let mut counts: &mut [_] = &mut counts.into_iter().collect::<Vec<_>>();
    counts.sort_unstable_by_key(|(k, _)| *k);
    while !counts.is_empty() {
        if counts.len() < n {
            return false;
        }
        let (k, v) = counts[0];
        for i in 0..n {
            let (k1, ref mut v1) = counts[i];
            if k1 != k + i as i32 || *v1 < v {
                return false;
            } else {
                *v1 -= v
            }
        }
        while let Some((_, 0)) = counts.first() {
            counts = &mut counts[1..];
        }
    }
    true
}
