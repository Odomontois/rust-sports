use std::collections::HashMap;
use Res::*;

enum Res {
    Edges(Vec<i32>),
    Found(Option<(i32, usize)>),
}

struct Search { nums: Vec<i32>, cache: HashMap<i32, Res> }

impl Search {
    #[allow(dead_code)]
    fn new(nums: Vec<i32>) -> Search {
        let cache = nums.iter().cloned().map(|n|
            (n, Edges(nums.iter().cloned().filter(|&m| m % n == 0 && m > n).collect()))
        ).collect();
        Search { nums, cache }
    }
    #[allow(dead_code)]
    fn find_best(&mut self, es: impl IntoIterator<Item=i32>) -> Option<(i32, usize)> {
        es.into_iter().map(|i: i32| (i, self.calc_best(i).map(|(_, s)| s + 1).unwrap_or(1))).max_by_key(|&(_, l)| l)
    }
    #[allow(dead_code, mutable_borrow_reservation_conflict)]
    fn calc_best(&mut self, start: i32) -> Option<(i32, usize)> {
        match self.cache.get(&start)? {
            Found(res) => *res,
            Edges(es) => {
                let best = self.find_best(es.clone());
                self.cache.insert(start, Found(best));
                best
            }
        }
    }
    #[allow(dead_code)]
    fn unwind(&mut self, mut start: i32) -> Vec<i32> {
        let mut res = vec![start];
        while let Some((next, _)) = self.calc_best(start) {
            res.push(next);
            start = next
        }
        res
    }
    #[allow(dead_code)]
    fn search(&mut self) -> Option<Vec<i32>> {
        let (best, _) = self.find_best(self.nums.clone())?;
        Some(self.unwind(best))
    }
}

#[allow(dead_code)]
pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
    Search::new(nums).search().unwrap_or(Vec::new())
}

#[allow(dead_code)]
pub fn sort_colors(nums: &mut Vec<i32>) {
    let counts = &mut [0; 3];
    for &i in nums.iter() { counts[i as usize] += 1 }
    nums.clear();
    for i in 0..3usize {
        nums.append(&mut vec![i as i32; counts[i]])
    }
}

#[allow(dead_code)]
pub fn remove_kdigits(num: String, k: i32) -> String {
    let mut ds: Vec<_> = num.bytes().into_iter().enumerate().collect();
    ds.sort_by_key(|&x| -(x.1 as i32));
    ds = ds[k as usize..].to_vec();
    ds.sort_by_key(|&x| x.0);
    String::from_utf8(ds.into_iter().map(|x| x.1).collect()).unwrap()
}


