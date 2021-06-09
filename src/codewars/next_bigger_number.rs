use itertools::Itertools;

fn next_bigger_number(n: i64) -> i64 {
    let mut digs: Vec<_> = n.to_string().chars().collect();
    let dr = digs.iter().copied().rev();
    let suff = dr.tuple_windows().take_while(|&(n, p)| n <= p).count();
    if digs.len() <= suff + 1 {
        return -1;
    }
    let i = digs.len() - 2 - suff;
    let next = digs[i + 1..].iter().copied().filter(|&c| c > digs[i]).min().unwrap();
    let next_idx = digs.len() - 1 - digs.iter().rev().position(|&c| c == next).unwrap();
    digs.swap(i, next_idx);
    digs[i + 1..].sort();
    digs.into_iter().collect::<String>().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(21, next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071, next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
        assert_eq!(153233478, next_bigger_number(152874333));
    }
}
