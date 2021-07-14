use std::collections::HashMap;
use crate::data::combinations::Combinations;

pub fn min_number_of_semesters(n: i32, dependencies: Vec<Vec<i32>>, k: i32) -> i32 {
    let deps = dependencies.into_iter().map(|v| (v[0] as usize, v[1] as usize));
    let mut courses = Courses::new(n as usize, deps, k as usize);
    courses.calc().unwrap_or(254) as i32
}

#[derive(Debug)]
struct Courses { cache: HashMap<u16, Option<u8>>, req: Vec<usize>, seq: Vec<Vec<usize>>, ready: u16, done: u16, k: usize }

impl Courses {
    fn new(n: usize, deps: impl Iterator<Item=(usize, usize)>, k: usize) -> Courses {
        let mut seq = vec![vec![]; n];
        let mut req = vec![0; n];
        let mut ready = 0;
        for (r, s) in deps {
            seq[r - 1].push(s - 1);
            req[s - 1] += 1;
        }
        for i in 0..n {
            if req[i] == 0 {
                ready ^= 1 << i;
            }
        }
        Courses { cache: HashMap::new(), done: 0, seq, req, ready, k }
    }

    fn ready_vec(&self) -> Vec<usize> {
        (0..16).filter(|&i| (self.ready & (1 << i)) != 0).collect()
    }

    fn apply(&mut self, comb: &Vec<usize>, un: bool) {
        // println!("{:?} {:?} {:b}", self, comb, self.ready);
        let (add, check) = if un { (2, 1) } else { (0, 0) };
        for &i in comb {
            self.done ^= 1 << i;
            self.ready ^= 1 << i;
            for &j in &self.seq[i] {
                self.req[j] += add;
                self.req[j] -= 1;
                if self.req[j] == check {
                    self.ready ^= 1 << j;
                }
            }
        }
    }


    fn calc(&mut self) -> Option<u8> {
        if let Some(&res) = self.cache.get(&self.done) { return res; }
        if self.done + 1 == 1 << self.req.len() { return Some(0); }
        let ready_vec = self.ready_vec();
        let take = self.k.min(ready_vec.len());
        let res = Combinations::vec_copy(ready_vec, take).filter_map(|comb| {
            self.apply(&comb, false);
            let res = self.calc();
            self.apply(&comb, true);
            res.map(|x| x + 1)
        }).min();
        self.cache.insert(self.done, res);
        res
    }
}

#[test]
fn test() {
    fn check(deps: &[[i32; 2]], n: i32, k: i32, res: i32) {
        let dependencies = deps.iter().map(|i| i.into_iter().cloned().collect()).collect();
        assert_eq!(min_number_of_semesters(n, dependencies, k), res)
    }

    // check(&[[2, 1], [3, 1], [1, 4]], 4, 2, 3);
    check(&[[2, 1], [3, 1], [4, 1], [1, 5]], 5, 2, 4);
    // check(&[], 1, 2, 1);
}
