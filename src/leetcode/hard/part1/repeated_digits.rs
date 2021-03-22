use std::{iter::successors, usize};

struct Repeated {
    fact: Vec<i32>,
}

pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
    n - Repeated::new().non_dup_digs(n)
}

impl Repeated {
    fn new() -> Self {
        let mut xs = 1..=9;
        let fact = successors(Some(1), |&p| xs.next().map(|k| p * k)).collect();

        Repeated { fact }
    }

    fn p(&self, n: usize, k: usize) -> i32 {
        self.fact[n] / self.fact[n - k]
    }

    fn non_dup_full(&self, n: usize) -> i32 {
        (1..=n).map(|k| 9 * self.p(9, k - 1)).sum()
    }

    fn non_dup_digs(&self, n: i32) -> i32 {
        let bs: Vec<_> = format!("{}", n).as_str().bytes().map(|b| b - '0' as u8).collect();
        let f = self.non_dup_full(bs.len() - 1);
        let it = self.digs_iter(&bs, vec![]);
        it + f
    }

    fn digs_iter(&self, n: &[u8], mut prev: Vec<u8>) -> i32 {
        if n.is_empty() {
            return 1;
        }
        let exclude = prev.iter().filter(|&&x| x < n[0]).count() as i32 + prev.is_empty() as i32;
        let below = (n[0] as i32 - exclude) * self.p(9 - prev.len(), n.len() - 1);
        let follow = if prev.iter().all(|&d| d != n[0]) {
            prev.push(n[0]);
            self.digs_iter(&n[1..], prev)
        } else {
            0
        };
        below + follow
    }
}

#[test]
fn check_non_dup() {
    assert_eq!(num_dup_digits_at_most_n(20), 1);
    assert_eq!(num_dup_digits_at_most_n(100), 10);
    assert_eq!(num_dup_digits_at_most_n(1000), 262);
    assert_eq!(num_dup_digits_at_most_n(3482), 1523);
}
