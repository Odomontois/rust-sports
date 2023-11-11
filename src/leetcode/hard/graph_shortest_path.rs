use std::{convert::Infallible, ops::Index};
#[derive(Debug)]
struct Graph {
    best: Vec<Vec<u32>>,
}
trait MyIndex: Index<usize, Output = i32> {}
impl<A: Index<usize, Output = i32>> MyIndex for A {}
impl Graph {
    fn new<A: MyIndex>(n: i32, edges: Vec<A>) -> Self {
        let n = n as usize;
        let mut graph = Self {
            best: vec![vec![u32::MAX; n]; n],
        };
        for (i, j, d) in edges.into_iter().map(Self::unpack_edge) {
            graph.best[i][j] = d;
        }
        for k in 0..n {
            graph.check(k, k, 0);
        }
        graph
    }

    fn add_edge<A: MyIndex>(&mut self, edge: A) {
        let (i, j, d) = Self::unpack_edge(edge);
        self.check(i, j, d);
    }

    fn shortest_path(&mut self, i: i32, j: i32) -> i32 {
        self.best_of(i as usize, j as usize).map_or(-1, |i| i as i32)
    }
    fn best_of(&self, i: usize, j: usize) -> Option<u32> {
        let q = self.best[i][j];
        (q != u32::MAX).then(|| q)
    }
    fn unpack_edge(v: impl MyIndex) -> (usize, usize, u32) {
        (v[0] as usize, v[1] as usize, v[2] as u32)
    }

    fn indices(&self) -> impl Iterator<Item = usize> + 'static {
        0..self.best.len()
    }

    fn check_one(&mut self, i: usize, j: usize, d: u32) -> Option<()> {
        let q = &mut self.best[i][j];
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
    let mut g = Graph::new(4, vec![[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]);
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
