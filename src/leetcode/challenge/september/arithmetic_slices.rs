use std::collections::HashMap;

pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let mut counts = vec![HashMap::new(); nums.len()];
    for (i, &num) in nums.iter().enumerate() {
        for (j, &prev) in nums[..i].iter().enumerate() {
            let d = num as i64 - prev as i64;
            let [cj, sj] = counts[j].get(&d).copied().unwrap_or([0, 0]);
            let [ci, si] = counts[i].entry(d).or_insert([0, 0]);
            *ci += 1;
            *si += sj + cj;
        }
    }
    counts.iter().flat_map(|m| m.values()).map(|[_, x]| *x).sum()
}

#[cfg(test)]
fn check<const N: usize>(exp: i32, arg: [i32; N]) {
    assert_eq!(exp, number_of_arithmetic_slices(arg.to_vec()))
}

#[test]
fn test1() {
    check(7, [2, 4, 6, 8, 10])
}

#[test]
fn test2() {
    check(16, [7, 7, 7, 7, 7])
}

#[test]
fn test3() {
    check(48, [7, 7, 7, 7, 7, 7, 8, 9])
}

#[test]
fn test4() {
    check(4, [7, 7, 7, 8, 9])
}
