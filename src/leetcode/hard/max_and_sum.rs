use std::vec;

pub fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
    let pn = 1 << nums.len();
    let cache = vec![vec![None; pn]; num_slots as usize];
    AndSum { nums, cache }.calc(pn - 1, num_slots as usize)
}

struct AndSum {
    cache: Vec<Vec<Option<i32>>>,
    nums: Vec<i32>,
}

impl AndSum {
    fn calc(&mut self, set: usize, slots: usize) -> i32 {
        if slots == 0 || set == 0 {
            return  0;
        }
        if let Some(cached) =  self.cache[slots - 1][set]{
            return cached
        }
        
        todo!()
    }
}
