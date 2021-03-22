use crate::data::du::DisjointUnion;

pub fn distance_limited_paths_exist(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    Distance::new(n as usize, queries).go(edge_list)
}

type Triple = (usize, usize, u64);

fn sorted_with<A, B: Ord>(mut it: Vec<A>, f: impl Fn(&A) -> B) -> Vec<A> {
    it.sort_by_key(f);
    it
}

fn triple(v: Vec<i32>) -> Triple { (v[0] as usize, v[1] as usize, v[2] as u64) }

fn from_vec(v: Vec<Vec<i32>>) -> Vec<Triple> { v.into_iter().map(triple).collect() }

#[derive(Debug)]
struct Distance { du: DisjointUnion, good: Vec<bool>, queries: Vec<(usize, Triple)> }

impl Distance {
    fn new(n: usize, queries: Vec<Vec<i32>>) -> Self {
        let mut queries: Vec<_> = queries.into_iter().map(triple).enumerate().collect();
        queries.sort_by_key(|(_, (_, _, cost))| std::cmp::Reverse(*cost));
        Distance { du: DisjointUnion::new(n), good: vec![false; queries.len()], queries }
    }
    fn finalize(&mut self, bound: u64) {
        while self.queries.last().iter().any(|(_, (_, _, cost))| *cost <= bound) {
            if let Some((p, (x, y, _))) = self.queries.pop() {
                self.good[p] = self.du.joined(x, y);
            }
        }
    }
    fn go(mut self, edge_list: Vec<Vec<i32>>) -> Vec<bool> {
        for (a, b, cost) in sorted_with(from_vec(edge_list), |v| v.2) {
            self.finalize(cost);
            self.du.join(a, b);
        }
        self.finalize(std::u64::MAX);
        self.good
    }
}

#[test]
fn test() {
    fn to_vec_vec(xs: &[[i32; 3]]) -> Vec<Vec<i32>> {
        xs.iter().map(|v| v.iter().copied().collect()).collect()
    }
    fn check(n: i32, edge_list: &[[i32; 3]], queries: &[[i32; 3]], exp: &[bool]) {
        assert_eq!(
            distance_limited_paths_exist(n, to_vec_vec(edge_list), to_vec_vec(queries)),
            exp
        )
    }

    // check(3,
    //       &[[0, 1, 2], [1, 2, 4], [2, 0, 8], [1, 0, 16]],
    //       &[[0, 1, 2], [0, 2, 5], [0, 2, 4], [1, 2, 7]],
    //       &[false, true, false, true],
    // );
    //
    // check(5,
    //       &[[0, 1, 10], [1, 2, 5], [2, 3, 9], [3, 4, 13]],
    //       &[[0, 4, 14], [1, 4, 13], [1, 3, 11], [1, 3, 10], [0, 3, 11], [0, 3, 10]],
    //       &[true, false, true, true, true, false]);

    check(13,
          &[[9, 1, 53], [3, 2, 66], [12, 5, 99], [9, 7, 26], [1, 4, 78], [11, 1, 62], [3, 10, 50], [12, 1, 71], [12, 6, 63], [1, 10, 63], [9, 10, 88], [9, 11, 59], [1, 4, 37], [4, 2, 63], [0, 2, 26], [6, 12, 98], [9, 11, 99], [4, 5, 40], [2, 8, 25], [4, 2, 35], [8, 10, 9], [11, 9, 25], [10, 11, 11], [7, 6, 89], [2, 4, 99], [10, 4, 63]],
          &[[9, 7, 65], [9, 6, 1], [4, 5, 34], [10, 8, 43], [3, 7, 76], [4, 2, 15], [7, 6, 52], [2, 0, 50], [7, 6, 62], [1, 0, 81], [4, 5, 35], [0, 11, 86], [12, 5, 50], [11, 2, 2], [9, 5, 6], [12, 0, 95], [10, 6, 9], [9, 4, 73], [6, 10, 48], [12, 0, 91], [9, 10, 58], [9, 8, 73], [2, 3, 44], [7, 11, 83], [5, 3, 14], [6, 2, 33]],
          &[true, false, false, true, true, false, false, true, false, true, false, true, false, false, false, true, false, true, false, true, true, true, false, true, false, false]);
}


