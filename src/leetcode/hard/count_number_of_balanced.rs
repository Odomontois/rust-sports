pub fn count_balanced_permutations(num: String) -> i32 {
    let (total, counts) = sum_count(&num);
    let mut fact = Facts::new();
    if total % 2 == 1 {
        return 0;
    }
    let n = num.len() as i32;
    let target = Distribs::calc(&num);
    let mut res = 0;
    for t in target {
        let mut comp = [0; 10];
        for (d, c) in t.iter().enumerate() {
            comp[d] = counts[d] - c
        }
        res += (fact.perms(t, n / 2) * fact.perms(comp, (n + 1) / 2)) % MOD
    }
    (res % MOD) as i32
}

struct Facts(Vec<i32>);

const MOD: i64 = 1_000_000_007;
impl Facts {
    fn new () -> Self {
        Facts(vec![1])
    }
    fn fact(&mut self, n: i32) -> i64 {
        let Facts(fs) = self;
        while fs.len() <= n as usize {
            let last = fs.last().copied().unwrap_or(1) as i64;
            fs.push(((last * fs.len() as i64) % MOD) as i32);
        }
        fs[n as usize] as i64
    }

    fn comb(&mut self, n: i32, k: i32) -> i64 {
        return (self.fact(n) * reci((self.fact(k) * self.fact(n - k)) % MOD)) % MOD;
    }

    fn perms(&mut self, counts: [i8; 10], mut sum: i32) -> i64 {
        let mut cur = 1;
        for count in counts {
            cur = (cur * self.comb(sum, count as i32)) % MOD;
            sum -= count as i32;
        }
        return cur;
    }
}

fn euclid(a: i64, b: i64) -> (i64, i64) {
    if a == 1 {
        return (1, 0);
    }
    let (x, y) = euclid(b, a % b);
    // x b + y r = 1, d b + r = a, r = a - d b
    // x b + y (a - d b) = 1
    // y a + (x - y d) b = 1
    (y, x - y * (a / b))
}

fn reci(a: i64) -> i64 {
    let (x, _) = euclid(a, MOD);
    return (x + MOD) % MOD;
}

fn sum_count(num: &str) -> (i32, [i8; 10]) {
    let (mut total, mut counts) = <(i32, [i8; 10])>::default();
    for c in num.bytes() {
        let d = (c - b'0') as i32;
        counts[d as usize] += 1;
        total += d;
    }
    return (total, counts);
}

struct Distribs {
    dest: Vec<[i8; 10]>,
    cur: [i8; 10],
    counts: [i8; 10],
    csums: [i8; 10],
    dig_sums: [i32; 10],
}

impl Distribs {
    fn calc(num: &str) -> Vec<[i8; 10]> {
        let (total, counts) = sum_count(num);
        let mut csums = counts;
        let mut dig_sums = [0; 10];
        for d in 1..10 {
            csums[d] += csums[d - 1];
            dig_sums[d] = dig_sums[d - 1] + d as i32 * counts[d] as i32;
        }
        let mut dists = Self { dest: Vec::new(), cur: [0; 10], counts, csums, dig_sums };
        dists.recur(9, total / 2, num.len() as i32 / 2);
        dists.dest
    }

    fn recur(&mut self, d: i32, s: i32, rest: i32) {
        if d == -1 && s == 0 && rest == 0 {
            self.dest.push(self.cur);
            return;
        } else if d == -1 || (self.csums[d as usize] as i32) < rest || self.dig_sums[d as usize] < s {
            return;
        }

        for i in 0.. {
            if i * d > s || i > self.counts[d as usize] as i32 || i > rest {
                break;
            }
            self.cur[d as usize] = i as i8;
            self.recur(d - 1, s - i * d, rest - i)
        }
    }
}


#[test]
fn example1(){
    assert_eq!(count_balanced_permutations("123".to_string()), 2);
}
