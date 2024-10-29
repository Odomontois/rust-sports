pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
    let nums = std::collections::HashSet::<i32>::from_iter(nums);

    let b1 = nums.contains(&2);
    let b2 = nums.contains(&4);
    let b3 = nums.contains(&16);
    let b4 = nums.contains(&256);
    let b5 = nums.contains(&65536);
    let b234 = b2 && b3 && b4;

    if b1 && b234 && b5 {
        return 5;
    }

    let t1 = nums.contains(&3);
    let t2 = nums.contains(&9);
    let t3 = nums.contains(&81);
    let t4 = nums.contains(&6561);

    if (t1 && t2 && t3 && t4) || (b234 && (b1 || b4)) {
        return 4;
    }

    let triples = (2..18).any(|c| nums.contains(&c) && nums.contains(&(c * c)) && nums.contains(&(c * c * c * c)));
    if triples {
        return 3;
    }
    let doubles = (2..320).any(|c| nums.contains(&c) && nums.contains(&(c * c)));
    if doubles {
        return 2;
    }
    -1
}
