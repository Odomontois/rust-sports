#[cfg(test)]
use itertools::Itertools;
#[test]
fn solve() {
    use self::tests::*;
    const N: usize = 3;
    let Some(best) = strategies(N).max_by_key(|st| score(st, N)) else {
        return;
    };
    let k = (0..2 * N).permutations(2 * N).count() as f64;
    let score = score(&best, N);
    let q = score as f64 / k as f64;

    println!("best score {q} ({score} / {k}) for {best:?}");
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use std::iter::{once, repeat};

    pub fn score(st: &Straregy, n: usize) -> usize {
        (0..2 * n).permutations(2 * n).filter(|p| survived(st, p)).count()
    }

    fn survived(st: &Straregy, nums: &[usize]) -> bool {
        st.iter().enumerate().all(|(i, s)| s.iter().any(|&j| nums[j] == i))
    }

    type Straregy = Vec<Vec<usize>>;

    pub fn strategies(n: usize) -> impl Iterator<Item = Straregy> {
        let first: Vec<Vec<usize>> = vec![(0..n).collect()];
        let rest: Vec<Vec<usize>> = (0..2 * n).combinations(n).collect();
        once(first)
            .chain(repeat(rest).take(2 * n - 1))
            .multi_cartesian_product()
    }
}
