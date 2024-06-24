pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    use std::collections::BTreeMap;
    let mut it = nums.iter().copied();
    let it1 = it.clone().scan((BTreeMap::new(), 0), |(br, c), x| {
        *br.entry(x).or_insert(0) += 1;
        *c += 1;
        while br.last_key_value()?.0 - br.first_key_value()?.0 > limit {
            *c -= 1;
            let k = it.next()?;
            let v = br.entry(k).or_insert(0);
            if *v == 1 {
                br.remove(&k);
            } else {
                *v -= 1;
            }
        }
        Some(*c)
    });
    it1.max().map_or(0, |x| x as i32)
}
