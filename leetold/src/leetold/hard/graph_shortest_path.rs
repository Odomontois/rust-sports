use std::{convert::Infallible, ops::Index};
#[derive(Debug)]
struct Graph {
    n: usize,
    best: Vec<u32>,
}
trait Unpack {
    fn unpack(self) -> (usize, usize, u32);
}
impl<A: Index<usize, Output = i32>> Unpack for A {
    fn unpack(self) -> (usize, usize, u32) {
        (self[0] as usize, self[1] as usize, self[2] as u32)
    }
}
impl Graph {
    fn new(n: i32, edges: impl IntoIterator<Item = impl Unpack>) -> Self {
        let n = n as usize;
        let mut graph = Self {
            n,
            best: vec![u32::MAX; n * n],
        };
        for (i, j, d) in edges.into_iter().map(<_>::unpack) {
            graph.best[i * n + j] = d;
        }
        for k in 0..n {
            graph.check(k, k, 0);
        }
        graph
    }

    fn add_edge(&mut self, edge: impl Unpack) {
        let (i, j, d) = edge.unpack();
        self.check(i, j, d);
    }

    fn shortest_path(&mut self, i: i32, j: i32) -> i32 {
        self.best_of(i as usize, j as usize).map_or(-1, |i| i as i32)
    }
    fn best_of(&self, i: usize, j: usize) -> Option<u32> {
        let q = self.best[i * self.n + j];
        (q != u32::MAX).then(|| q)
    }

    fn indices(&self) -> impl Iterator<Item = usize> + 'static {
        0..self.n
    }

    fn check_one(&mut self, i: usize, j: usize, d: u32) -> Option<()> {
        let q = &mut self.best[i * self.n + j];
        (*q > d).then(|| {
            *q = d;
        })
    }

    fn check(&mut self, i: usize, j: usize, d: u32) -> Option<Infallible> {
        self.check_one(i, j, d)?;
        for a in self.indices() {
            if let Some(ai) = self.best_of(a, i) {
                for b in self.indices() {
                    if let Some(jb) = self.best_of(j, b) {
                        self.check_one(a, b, ai + d + jb);
                    }
                }
            }
        }
        None
    }
}

#[test]
fn example1() {
    let mut g = Graph::new(4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]);
    assert_eq!(g.shortest_path(3, 2), 6);
    assert_eq!(g.shortest_path(0, 3), -1);
    g.add_edge([1, 3, 4]);
    assert_eq!(g.shortest_path(0, 3), 6);
}
/*
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */

#[test]
fn check() {
    println!("{}", std::mem::size_of::<Option<Infallible>>())
}
