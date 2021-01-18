struct Solution;

pub fn gmmmmm(n: i32) -> i32 {
    let mut nums = vec![0; n as usize + 1];
    nums[1] = 1;
    let mut maxes = vec![(0, 0), (1, 1)];
    for i in 2..nums.len() {
        nums[i] = if i % 2 == 0 { nums[i / 2] } else { nums[i / 2] + nums[i / 2 + 1] };
        if nums[i] > maxes.last().unwrap().0 { maxes.push((nums[i], i)) }
    };
    for &(x, i) in &maxes {
        println!("{} {:b} : {} {:b}", x, x, i, i)
    }
    println!("{:?}", maxes.iter().map(|p| p.1).collect::<Vec<_>>());
    println!("{:?}", nums);
    0
}

pub fn get_maximum_generated(n: i32) -> i32 {
    let mut nums = vec![0; n as usize + 1];
    nums[1] = 1;
    for i in 2..nums.len() {
        nums[i] = if i % 2 == 0 { nums[i / 2] } else { nums[i / 2] + nums[i / 2 + 1] };
    };
    nums.into_iter().max().unwrap_or(0)
}

#[test]
fn gmg() {
    gmmmmm(100);
}

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        let median = med_of_meds(&mut nums);
        let (g, ge) = nums.iter().copied().fold((0, 0), |(g, ge), x|
            (g + (x > median) as i32, ge + (x >= median) as i32),
        );
        if g >= k {
            Self::find_kth_largest(nums.into_iter().filter(|&x| x > median).collect(), k)
        } else if ge >= k { median } else {
            Self::find_kth_largest(nums.into_iter().filter(|&x| x < median).collect(), k - ge)
        }
    }
}

fn med_of_meds<A: Ord + Copy>(xs: &mut [A]) -> A {
    if xs.len() == 1 { return xs[0]; }
    let mut medians: Vec<A> = xs.chunks_mut(5)
        .map(|w| if w.len() < 5 { w[0] } else {
            w.sort();
            w[2]
        }).collect();
    med_of_meds(&mut medians)
}

#[test]
fn check() {
    Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2);
}

static mut C: [[u64; 50]; 50] = [[0; 50]; 50];

pub fn count_vowel_strings(n: i32) -> i32 {
    comb(n as usize + 4, 4) as i32
}

fn comb(n: usize, k: usize) -> u64 {
    unsafe {
        if k == 0 || k == n || n == 0 { 1 } else {
            if C[n - 1][k - 1] == 0 {
                C[n - 1][k - 1] = comb(n - 1, k - 1) + comb(n - 1, k)
            }
            C[n - 1][k - 1]
        }
    }
}