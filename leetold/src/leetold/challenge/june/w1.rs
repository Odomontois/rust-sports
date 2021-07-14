use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::once;
use std::vec;

pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let s3 = s3.as_bytes();
    let mut possible = vec![vec![false; s2.len() + 1]; s1.len() + 1];
    if s3.len() != s1.len() + s2.len() {
        return false;
    }

    for i1 in 0..=s1.len() {
        for i2 in 0..=s2.len() {
            let i3 = i1 + i2;
            possible[i1][i2] = (i1 > 0 && possible[i1 - 1][i2] && s1[i1 - 1] == s3[i3 - 1])
                || (i2 > 0 && possible[i1][i2 - 1] && s2[i2 - 1] == s3[i3 - 1])
                || (i1 == 0 && i2 == 0);
        }
    }
    possible[s1.len()][s2.len()]
}

#[test]
fn test_interleave() {
    fn check(s1: &str, s2: &str, s3: &str) -> bool {
        is_interleave(s1.to_string(), s2.to_string(), s3.to_string())
    }
    // assert!(check("aabcc", "dbbca", "aadbbcbcac"));
    // assert!(!check("aabcc", "dbbca", "aadbbbaccc"));
    assert!(check("", "", ""));
}

pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let max_piece = |mut v: Vec<_>, l| {
        v.sort();
        let v = vec![vec![0], v, vec![l]].concat();
        v.windows(2).map(|w| w[1] - w[0]).max().unwrap_or(0)
    };
    (max_piece(horizontal_cuts, h) as u64 * max_piece(vertical_cuts, w) as u64 % (10u64.pow(9) + 7)) as i32
}

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut seen = vec![vec![false; m]; n];
    let mut q = VecDeque::new();
    let grid = &grid;
    let mut count = move |i: usize, j: usize, u| {
        if u == 0 || seen[i][j] {
            return None;
        }
        let mut res = 1;
        q.push_back((i, j));
        seen[i][j] = true;
        while let Some((x, y)) = q.pop_front() {
            for (dx, dy) in &[(2, 1), (0, 1), (1, 2), (1, 0)] {
                if (1..=n).contains(&(x + dx)) && (1..=m).contains(&(y + dy)) {
                    let x = x + dx - 1;
                    let y = y + dy - 1;
                    if !seen[x][y] && grid[x][y] == 1 {
                        seen[x][y] = true;
                        res += 1;
                        q.push_back((x, y));
                    }
                }
            }
        }
        Some(res)
    };

    grid.iter()
        .enumerate()
        .filter_map(|(i, v)| v.iter().enumerate().filter_map(|(j, &x)| count(i, j, x)).max())
        .max()
        .unwrap_or(0)
}

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let target: u16 = target.parse().unwrap_or(0);
    let mut deadend = vec![false; 10000];
    for d in deadends.iter().flat_map(|s| s.parse::<usize>()) {
        deadend[d] = true;
    }
    if deadend[0] {
        return -1;
    }
    deadend[0] = true;
    let mut q = VecDeque::new();
    q.push_back((0u16, 0));
    while let Some((n, s)) = q.pop_front() {
        if n == target {
            return s;
        }
        deadend[n as usize] = true;
        for m in nexts(n) {
            if deadend[m as usize] {
                continue;
            }
            deadend[m as usize] = true;
            q.push_back((m, s + 1));
        }
    }

    -1
}

pub fn nexts(x: u16) -> impl Iterator<Item = u16> {
    [1u16, 10, 100, 1000].iter().flat_map(move |&p| {
        let d = x % (p * 10) / p;
        once((d + 9) % 10)
            .chain(once((d + 1) % 10))
            .map(move |y| x - d * p + y * p)
    })
}

#[test]
fn test_lock() {
    fn check(deadends: &[&str], target: &str, exp: i32) {
        assert_eq!(
            open_lock(deadends.iter().map(|s| s.to_string()).collect(), target.to_string()),
            exp
        )
    }
    check(&["0201", "0101", "0102", "1212", "2002"], "0202", 6);
    check(&["8888"], "0009", 1);
    check(&["8888"], "9000", 1);
    check(
        &["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"],
        "8888",
        -1,
    );
    check(&["0000"], "8888", -1);
}

pub fn max_performance(_: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
    let mut best = BinaryHeap::new();
    let mut xs: Vec<_> = speed.into_iter().zip(efficiency.into_iter()).collect();
    xs.sort_by_key(|(_, eff)| Reverse(*eff));
    let mut sum = 0;
    (xs.into_iter()
        .map(|(sp, eff)| {
            sum += sp as i64;
            best.push(Reverse(sp));
            if best.len() as i32 > k {
                sum -= best.pop().unwrap_or_default().0 as i64
            }
            sum * eff as i64
        })
        .max()
        .unwrap_or(0)
        % (10i64.pow(9) + 7)) as i32
}

#[test]
fn test_performance() {
    fn check(speeds: &[i32], effs: &[i32], k: i32, exp: i32) {
        assert_eq!(
            max_performance(speeds.len() as i32, speeds.to_vec(), effs.to_vec(), k),
            exp
        )
    }

    check(&[2, 10, 3, 1, 5, 8], &[5, 4, 3, 9, 7, 2], 2, 60);
    check(&[2, 10, 3, 1, 5, 8], &[5, 4, 3, 9, 7, 2], 3, 68);
    check(&[2, 10, 3, 1, 5, 8], &[5, 4, 3, 9, 7, 2], 4, 72);
}

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let [p, pp] = cost.iter().rev().fold([0, 0], |[pp, p], &c| [p, c + p.min(pp)]);
    p.min(pp)
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<_, Option<i32>> = nums.iter().map(|&n| (n, None)).collect();
    nums.iter()
        .map(|&n| {
            let (mut start, mut fill) = (n, n);
            while let Some(v) = map.get(&start) {
                if let &Some(m) = v {
                    start = m;
                    break;
                }
                start -= 1;
                fill -= 1;
            }
            for i in fill..=n {
                map.insert(i, Some(start));
            }

            n - start
        })
        .max()
        .unwrap_or(0)
}

#[test]

fn test_conseq() {
    assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
}
