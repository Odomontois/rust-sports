pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let mut max = 0;
    let mut patches = 0;
    for num in nums {
        while max < num as i64 - 1 && max < n as i64 {
            max = 2 * max + 1;
            patches += 1;
        }
        if max >= n as i64 {
            return patches;
        }
        max += num as i64
    }
    while max < n as i64 {
        max = 2 * max + 1;
        patches += 1
    }
    patches
}
