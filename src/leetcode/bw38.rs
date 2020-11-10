use std::convert::TryFrom;
use std::ops::{Add, Mul};
#[allow(dead_code)]
pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut freq = vec![0; 300];
    fn idx(x: &i32) -> usize { (x + 150) as usize }
    for num in &nums {
        freq[idx(num)] += 1
    }
    let mut res = nums;
    res.sort_by_key(|x| (freq[idx(x)], -x));
    res
}
#[allow(dead_code)]
pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut xs = points.iter().map(|v| v[0]).collect::<Vec<_>>();
    xs.sort();
    xs.windows(2).flat_map(<&[i32; 2]>::try_from).map(|[x1, x2]| x2 - x1).max().unwrap_or(0)
}

#[allow(dead_code)]
pub fn count_substrings(s: String, t: String) -> i32 {
    let sc = s.as_bytes();
    let tc = t.as_bytes();
    let mut sum = 0;
    for i in 0..s.len() {
        for j in 0..t.len() {
            if sc[i] != tc[j] {
                let mut p = 1;
                let mut q = 1;
                while p <= i && p <= j && sc[i - p] == tc[j - p] { p += 1 }
                while q < s.len() - i && q < t.len() - j && sc[i + q] == tc[j + q] { q += 1 }
                sum += (p * q) as i32;
            }
        }
    }
    sum
}

#[derive(Copy, Clone)]
struct Answer { num: i64 }

static BASE: i64 = 1000_000_007;

impl Mul for Answer {
    type Output = Answer;
    fn mul(self, rhs: Self) -> Self {
        Answer { num: (self.num * rhs.num) % BASE }
    }
}

impl Add for Answer {
    type Output = Answer;
    fn add(self, rhs: Self) -> Self {
        Answer { num: (self.num + rhs.num) % BASE }
    }
}

#[allow(dead_code)]
pub fn num_ways(words: Vec<String>, target: String) -> i32 {
    let n = words[0].len();
    let m = target.len();
    let zero = Answer { num: 0 };
    let mut wm = vec![vec![zero; 128]; n];
    for word in &words {
        for (i, c) in word.as_bytes().iter().enumerate() {
            wm[i][*c as usize].num += 1;
        }
    }

    let mut dp = vec![vec![zero; m + 1]; n + 1];
    let tc = target.as_bytes();
    dp[n][m].num += 1;
    for wi in (0..n).rev() {
        dp[wi][m].num += 1;
        for ti in (0..m).rev() {
            dp[wi][ti] = wm[wi][tc[ti] as usize] * dp[wi + 1][ti + 1] + dp[wi + 1][ti]
        }
    }
    dp[0][0].num as i32
}