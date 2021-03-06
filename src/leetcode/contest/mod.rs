pub mod w214;
pub mod w215;
pub mod bw39;
mod bw40;
mod bw42;
mod bw47;

#[allow(dead_code)]
pub fn get_maximum_generated(n: i32) -> i32 {
    let n1 = n as usize + 1;
    let mut nums = vec![0; 2.max(n1)];
    nums[1] = 1;
    for i in 2..n1 {
        nums[i] = if i % 2 == 0 { nums[i / 2] } else { nums[i / 2] + nums[i / 2 + 1] }
    }
    nums.into_iter().take(n1).max().unwrap_or(0)
}