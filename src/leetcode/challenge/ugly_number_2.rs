const NUMS: [i32; 1700] = generate_nums();

#[test]
fn kek(){
    println!("{:?}", NUMS);
}
pub fn nth_ugly_number(n: i32) -> i32 {
    NUMS[n as usize - 1]
}

const fn generate_nums<const N: usize>() -> [i32; N] {
    let mut res = [0; N];
    res[0] = 1;
    let (res, k) = multiply_all(res, 1, 2);
    let (res, k) = multiply_all(res, k, 3);
    let (res, _) = multiply_all(res, k, 5);
    sort_nums(res)
}

const fn multiply_all<const N: usize>(mut nums: [i32; N], k: usize, p: i32) -> ([i32; N], usize) {
    let mut u = k;
    let mut i = 0;
    while i < k {
        let mut c = nums[i] as i64 * p as i64;
        while c < i32::MAX as i64 && u < N{
            nums[u] = c as i32;
            u += 1;
            c *= p as i64;
        }
        i += 1;
    }
    (nums, u)
}

const fn sort_nums<const N: usize>(mut nums: [i32; N]) -> [i32; N] {
    let mut i = N - 1;
    while i > 0 {
        let mut j = 0;
        while j < i {
            if nums[j] > nums[j + 1]  && nums[j + 1] != 0 {
                (nums[j], nums[j + 1]) = (nums[j + 1], nums[j]);
            }
            j += 1;
        }
        i -= 1;
    }
    nums
}
