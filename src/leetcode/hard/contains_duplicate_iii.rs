pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, id: i32, vd: i32) -> bool {
    let id = id as usize;
    let mut s = std::collections::BTreeMap::<i32, u32>::new();
    for (i, &n) in nums.iter().enumerate() {
        if s.range(n..).take(1).any(|(&x, _)| x - n <= vd) {
            return true;
        }
        if s.range(..=n).rev().take(1).any(|(&x, _)| n - x <= vd) {
            return true;
        }
        *s.entry(n).or_insert(0) += 1;
        if i >= id {
            let k = *s.entry(nums[i - id]).and_modify(|x| *x -= 1).or_default();
            if k == 0 {
                s.remove(&nums[i - id]);
            }
        }
    }
    false
}

#[test]
fn example1() {
    assert!(contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0))
}
