use std::collections::{HashMap, VecDeque};

pub fn second_highest(s: String) -> i32 {
    let digs = || s.chars().filter(|c| c.is_digit(10)).map(|c| c as i32 - '0' as i32);
    let best = digs().max();
    digs().filter(|&c| Some(c) != best).max().unwrap_or(-1)
}

struct AuthenticationManager {
    tokens: VecDeque<(i32, String)>,
    times: HashMap<String, (i32, u32)>,
    ttl: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuthenticationManager {
    fn new(ttl: i32) -> Self {
        Self {
            tokens: VecDeque::new(),
            times: HashMap::new(),
            ttl,
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.tokens.push_back((current_time, token_id.clone()));
        let (t, count) = self.times.entry(token_id).or_insert((current_time, 0));
        *t = current_time;
        *count += 1;
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        if let Some((t, _)) = self.times.get(&token_id) {
            if *t + self.ttl > current_time {
                self.generate(token_id, current_time);
            }
        }
    }

    fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        while let Some((t, _)) = self.tokens.front() {
            if *t + self.ttl > current_time {
                break;
            }
            let (_, id) = self.tokens.pop_front().unwrap();
            let (_, c) = self.times.get_mut(&id).unwrap();
            *c -= 1;
            if *c == 0 {
                self.times.remove(&id);
            }
        }
        self.times.len() as i32
    }
}

pub fn get_maximum_consecutive(mut coins: Vec<i32>) -> i32 {
    coins.sort();
    let mut prev = 0;
    for c in coins {
        if prev + 1 < c {
            return prev + 1;
        }
        prev += c
    }
    prev + 1
}

pub fn max_score(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = 1 << n;
    let mut res = vec![0; m];
    let gcds: Vec<Vec<i32>> = (0..n)
        .map(|i| (0..n).map(|j| gcd(nums[i], nums[j])).collect())
        .collect();
    for t in 0..m {
        let has = |&i: &usize| t & (1 << i) != 0;
        let k = (n as i32 - t.count_ones() as i32) / 2 + 1;
        res[t] = (0..n)
            .filter(has)
            .filter_map(|i| {
                (i + 1..n)
                    .filter(has)
                    .map(|j| gcds[i][j] * k + res[t ^ (1 << i) ^ (1 << j)])
                    .max()
            })
            .max()
            .unwrap_or(0)
    }
    res[m - 1]
}

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

#[test]
fn check(){
    assert_eq!(max_score(vec![1,2]), 1);
}
