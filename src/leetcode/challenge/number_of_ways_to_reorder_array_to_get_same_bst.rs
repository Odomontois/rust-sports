pub fn num_of_ways(nums: Vec<i32>) -> i32 {
    let mut fact = Fact::default();
    let mut params = vec![Some(nums)];
    let mut results = vec![];
    while let Some(p) = params.pop() {
        if let Some(mut vec) = p {
            if vec.is_empty() {
                results.push(1);
            } else {
                let fst = vec[0];
                let smaller: Vec<_> = vec.iter().filter(|&&x| x < fst).copied().collect();
                vec.retain(|&x| x > fst);
                let bigger = vec;
                results.push(fact.comb_sum(smaller.len(), bigger.len()));
                params.push(None);
                params.push(Some(smaller));
                params.push(Some(bigger));
            }
        } else {
            let r = results.drain(results.len() - 3..).fold(1, |x, y| mul([x, y]));
            results.push(r);
        }
    }
    ((results[0] + MOD - 1) % MOD) as i32
}

const MOD: i64 = 1_000_000_007;

fn mul<const N: usize>(xs: [i64; N]) -> i64 {
    xs.iter().fold(1, |acc, x| (acc * x) % MOD)
}

fn euclid(x: i64, y: i64) -> (i64, i64) {
    if x == 1 {
        return (1, 0);
    } else {
        let (a, b) = euclid(y, x % y);
        return (b, a - (x / y) * b);
    }
}

fn inv(x: i64) -> i64 {
    (euclid(MOD, x).1 + MOD) % MOD
}

#[derive(Default)]
struct Fact(Vec<i64>);
impl Fact {
    fn of(&mut self, i: usize) -> i64 {
        while self.0.len() <= i {
            let k = self.0.len().max(1) as i64;
            let last = self.0.last().copied().unwrap_or(1);
            self.0.push((last * k) % MOD);
        }
        self.0[i]
    }

    fn inv_of(&mut self, i: usize) -> i64 {
        inv(self.of(i))
    }

    fn comb_sum(&mut self, a: usize, b: usize) -> i64 {
        mul([self.of(a + b), self.inv_of(a), self.inv_of(b)])
    }
}

#[test]
fn example1() {
    assert_eq!(1, num_of_ways(vec![2, 1, 3]))
}

#[test]
fn example2() {
    assert_eq!(5, num_of_ways(vec![3, 4, 5, 1, 2]))
}

#[test]
fn example3() {
    assert_eq!(0, num_of_ways(vec![1, 2, 3]))
}

#[test]
fn wa1() {
    assert_eq!(19, num_of_ways(vec![3, 1, 2, 5, 4, 6]))
}

#[test]
#[ignore]
fn lol() {
    let nums = [5, 1, 564312, 2, 645];
    let invs = nums.map(inv);
    let origs: Vec<_> = invs.iter().zip(nums).map(|(x, y)| x * y % MOD).collect();
    println!("{nums:?}\n{invs:?}\n{origs:?}",);
}
