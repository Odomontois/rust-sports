use std::collections::HashMap;
use crate::data::combinations::Combinations;

pub fn min_number_of_semesters(n: i32, dependencies: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut courses = Courses::new(n as usize, dependencies.into_iter().map(|v| (v[0] as usize, v[1] as usize)));
    courses.calc(k as usize) as i32
}

struct Courses { cache: HashMap<u16, u8>, req: Vec<usize>, seq: Vec<Vec<usize>>, ready: Vec<usize>, done: u16 }

impl Courses {
    fn new(n: usize, deps: impl Iterator<Item=(usize, usize)>) -> Courses {
        let mut seq = vec![vec![]; n];
        let mut req = vec![0; n];
        let mut ready = vec![];
        for (r, s) in deps {
            seq[r].push(s);
            req[s] += 1;
        }
        for i in 0..n {
            if req[i] == 0 {
                ready.push(i)
            }
        }
        Courses { cache: HashMap::new(), done: 0, seq, req, ready }
    }
    fn calc(&mut self, k: usize) -> u8 {
        if let Some(&res) = self.cache.get(&self.done) { return res; }

        let take = k.min(self.ready.len());
        for comb in Combinations::slice(&self.ready, take) {

        }


        unimplemented!()
    }
}
