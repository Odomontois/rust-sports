use std::{
    collections::{HashMap, HashSet},
};

use crate::err::*;

struct Tree {
    edges: Vec<Vec<usize>>,
    freqs: Vec<u32>,
}
type Counts = HashMap<u32, u32>;

pub fn solution() -> S {
    let tree = parse()?;

    Ok(format!("{}", tree.walk_req() - 1))
}

fn parse() -> R<Tree> {
    let n = scanln_fmt!("{}", usize)?;
    let mut edges = vec![vec![]; n];
    for _ in 0..n - 1 {
        let (x, y) = scanln_fmt!("{} {}", usize, usize)?;
        edges[x - 1].push(y - 1);
        edges[y - 1].push(x - 1);
    }
    let freqs = scanln_fmt!("{/.*/}", String)?;
    let freqs = freqs.split(" ").map(|s| s.parse::<u32>()).collect::<RV<_, _>>()?;
    Ok(Tree { edges, freqs })
}
type Item = (usize, Counts, Vec<usize>);
impl Tree {
    fn init_sum(&self, i: usize) -> Item {
        (i, Some((self.freqs[i], 1)).into_iter().collect(), self.edges[i].clone())
    }
    fn walk_sum(&self) -> Counts {
        let mut seen = HashSet::<usize>::new();
        let mut stack = vec![self.init_sum(0)];
        while let Some((i, m, mut cs)) = stack.pop() {
            if let Some(c) = cs.pop() {
                stack.push((i, m, cs));
                if seen.insert(c) {
                    stack.push(self.init_sum(c));
                }
            } else {
                if let Some((p, mut pm, pcs)) = stack.pop() {
                    pm = merge(m, pm);
                    stack.push((p, pm, pcs));
                } else {
                    return m;
                }
            }
        }
        Counts::default()
    }
    fn init_req(&self, i: usize) -> (usize, Vec<usize>) {
        (i, self.edges[i].clone())
    }
    fn walk_req(&self) -> u32 {
        let cnts = self.walk_sum();
        let mut seen = HashSet::<usize>::new();
        let mut stack = vec![self.init_req(0)];
        let mut reqs: Counts = HashMap::new();
        let mut drops = 0;
        while let Some((i, mut cs)) = stack.pop() {
            if let Some(c) = cs.pop() {
                stack.push((i, cs));
                if seen.insert(c) {
                    stack.push(self.init_req(c));
                }
            } else {
                let f = self.freqs[i];
                let c = reqs.entry(f).or_insert(cnts[&f]);
                *c -= 1;
                if *c == 0 {
                    reqs.remove(&f);
                }
                if reqs.is_empty() {
                    drops += 1;
                }
                println!("{:?}", (i, &reqs));
            }
        }
        drops
    }
}

fn merge(mut m1: Counts, m2: Counts) -> Counts {
    if m1.len() < m2.len() {
        return merge(m2, m1);
    }
    for (k, v) in m2 {
        *m1.entry(k).or_insert(0) += v;
    }
    m1
}

