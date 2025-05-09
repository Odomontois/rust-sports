use std::collections::HashMap;

pub fn count_balanced_permutations(num: impl AsRef<str>) -> i32 {
    Perms::new(num.as_ref().bytes().map(|b| b - b'0')).solve() as i32
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct PermInput {
    digit: i16,
    remains: i16,
    sum: i16,
}
struct Perms {
    cache: HashMap<PermInput, i64>,
    comb: Vec<Vec<i64>>,
    counts: [i16; 10],
    len: i16,
    sum: i16,
}
impl Perms {
    const MOD: i64 = 1_000_000_007;

    fn init_combs(n: usize) -> Vec<Vec<i64>> {
        let mut comb = vec![vec![0; n + 1]; n + 1];
        for i in 0..=n {
            comb[i][0] = 1;
            for j in 1..=i {
                comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % Self::MOD;
            }
        }
        comb
    }

    fn comb(&self, n: i16, k: i16) -> i64 {
        self.comb[n as usize][k as usize] as i64
    }

    fn new(perm: impl IntoIterator<Item = u8>) -> Self {
        let mut counts = [0; 10];
        let mut len = 0;
        let mut sum = 0;
        for digit in perm {
            counts[digit as usize] += 1;
            len += 1;
            sum += digit as i16;
        }
        Self { cache: <_>::default(), comb: Self::init_combs(len as usize), counts, len, sum }
    }

    fn solve(&mut self) -> i64 {
        self.calc(self.len / 2, (self.len + 1) / 2, self.sum / 2, self.sum / 2, 9)
    }

    fn calc(&mut self, remains: i16, outside: i16, sum: i16, out_sum: i16, digit: i16) -> i64 {
        if digit == 0 {
            return if sum == 0 && remains <= self.counts[0] { 1 } else { 0 };
        }
        let input = PermInput { digit, remains, sum };
        if let Some(&res) = self.cache.get(&input) {
            return res;
        }

        let mut res = 0;

        let dcount = self.counts[digit as usize];
        let min = 0.max(dcount - outside).max(dcount - out_sum / digit);
        let max = dcount.min(remains).min(sum / digit);

        for i in min..=max {
            let out = dcount - i;
            let placement = (self.comb(remains, i) * self.comb(outside, out)) % Self::MOD;
            let rec = self.calc(
                remains - i,
                outside - out,
                sum - digit * i,
                out_sum - digit * out,
                digit - 1,
            );
            res = (res + placement * rec) % Self::MOD;
        }

        self.cache.insert(input, res);

        res
    }
}

#[test]
fn example1() {
    assert_eq!(count_balanced_permutations("123"), 2);
}

#[test]
fn example2() {
    assert_eq!(count_balanced_permutations("112"), 1);
}

#[test]
fn example3() {
    assert_eq!(count_balanced_permutations("12345"), 0);
}

#[test]
fn test1(){
    assert_eq!(count_balanced_permutations("1234"), 8)
}

#[test]
fn large_test1(){
    assert_eq!(count_balanced_permutations("512733941540437534034939558902869636198144375574624063"), 388835596)
}
#[test]
fn large_test2(){
    assert_eq!(count_balanced_permutations("5418743870162552655207847838680387883866911149998450118"), 500402485)
}
