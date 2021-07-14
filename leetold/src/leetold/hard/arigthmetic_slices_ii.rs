use std::collections::HashMap;

#[derive(Clone)]
struct Counter { counts: HashMap<(i64, bool), u64> }

impl Counter {
    fn new() -> Counter { Counter { counts: HashMap::new() } }
    fn add(&mut self, k: i64, b: bool, amt: u64) {
        if amt == 0 { return; }
        if let Some(a) = self.counts.get_mut(&(k, b)) {
            *a += amt;
        } else {
            self.counts.insert((k, b), amt);
        }
    }
    fn get(&self, k: i64, b: bool) -> u64 {
        self.counts.get(&(k, b)).cloned().unwrap_or(0)
    }
    fn sum(&self) -> u64 {
        self.counts.iter().filter_map(|(&(_, b), &c)| if b { Some(c) } else { None }).sum()
    }
}

pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
    let mut vs = vec![Counter::new(); a.len()];
    for i in 1..a.len() {
        for j in 0..i {
            let d = a[i] as i64 - a[j] as i64;
            let amt = vs[j].get(d, true) + vs[j].get(d, false);
            vs[i].add(d, true, amt);
            vs[i].add(d, false, 1);
        }
    }

    vs.into_iter().map(|c| c.sum()).sum::<u64>() as i32
}

#[test]
fn check(){
    assert_eq!(number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]), 7)
}