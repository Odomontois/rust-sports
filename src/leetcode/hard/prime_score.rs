const MOD: i64 = 10i64.pow(9) + 7;

pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let mut p = Primes::default();
    let scores: Vec<i32> = nums.iter().copied().map(|x| p.prime_score(x)).collect();
    // println!("{scores:?}");
    let scores_it = || scores.iter().copied().enumerate();
    let mono_back: Vec<_> = monotonic(scores_it().rev(), nums.len() as i32).collect();
    let mono_forward = monotonic(scores_it(), -1);
    let count = |i: usize, from: i32, to: i32| (i as i64 - from as i64) * (to as i64 - i as i64);
    let mut sizes: Vec<_> = mono_forward
        .zip(mono_back.into_iter().rev())
        .zip(nums.iter().enumerate())
        .map(|((from, to), (i, x))| (*x, count(i, from, to)))
        .collect();

    // println!("{sizes:?}");
    sizes.sort_unstable_by_key(|(x, _)| -*x);
    let mut k = k as i64;
    let res = &mut 1i64;
    for (x, s) in sizes {
        fast_pow(res, x as i64, s.min(k));
        k -= s;
        if k <= 0 {
            break;
        }
    }
    *res as i32
}

fn fast_pow(acc: &mut i64, mut x: i64, mut pow: i64) {
    while pow > 0 {
        if pow & 1 == 1 {
            *acc = (*acc * x) % MOD;
        }
        x = (x * x) % MOD;
        pow >>= 1;
    }
}

fn monotonic(nums: impl Iterator<Item = (usize, i32)>, edge: i32) -> impl Iterator<Item = i32> {
    let mut stack = Vec::<(usize, i32)>::new();
    nums.map(move |(i, x)| {
        while stack.last().filter(|&&(j, y)| y < x || y == x && j > i).is_some() {
            stack.pop();
        }
        let limit = stack.last().map_or(edge, |(j, _)| *j as i32);
        stack.push((i, x));
        limit
    })
}

struct Primes {
    primes: Vec<i32>,
    max: i32,
}

impl Default for Primes {
    fn default() -> Self {
        Self { primes: vec![2, 3], max: 3 }
    }
}

impl Primes {
    fn update(&mut self, max: i32) {
        while self.max * self.max < max {
            self.max += 2;
            if self
                .primes
                .iter()
                .take_while(|&p| p * p <= self.max)
                .all(|p| self.max % p != 0)
            {
                self.primes.push(self.max);
            }
        }
    }
    fn prime_score(&mut self, mut num: i32) -> i32 {
        self.update(num);
        let mut score = 0;
        for &p in &self.primes {
            if p * p > num {
                break;
            }
            score += (num % p == 0) as i32;
            while num % p == 0 {
                num /= p;
            }
        }
        score + (num > 1) as i32
    }
}

#[test]
fn example1() {
    assert_eq!(maximum_score(vec![8, 3, 9, 3, 8], 2), 81);
}

#[test]
fn example2() {
    assert_eq!(maximum_score(vec![19, 12, 14, 6, 10, 18], 3), 4788);
}

#[test]
fn test1() {
    assert_eq!(maximum_score(vec![6, 18, 30, 24, 10], 7), 869999853);
}

#[test]
fn test2() {
    assert_eq!(maximum_score(vec![91909, 21013, 79750, 46410], 10), 194309639);
}

#[test]
fn wa1() {
    assert_eq!(
        maximum_score(
            vec![
                1, 75696, 92150, 32867, 65704, 46410, 47384, 59753, 46889, 13860, 30030, 44027, 28210, 21930, 85470,
                56760, 47849, 33616, 10247, 66674, 91909, 21013, 79750, 46410, 1, 1, 20719, 67860, 66887, 26040, 31344,
                65729, 99298, 70764, 11332, 78540, 81510, 1, 28815, 16018, 1, 86394, 93765, 53025, 21491, 84417, 82460,
                88471, 86430, 12390, 21450, 1634, 36036, 55461, 43969, 26241, 1, 77550, 72930, 59466, 80410, 29387,
                67777, 1, 49266, 75748, 80388, 60060, 845, 99990, 55650, 90090, 46410, 1, 15331, 66990, 20190, 43451,
                98647, 55526, 18843, 92820, 44716, 1, 7069, 13466, 64680, 66850, 85281, 71610, 77887
            ],
            2889
        ),
        236209936
    );
}
