#[derive(Debug, Clone, Copy)]
enum UFNode {
    Root { size: usize, and: i32 },
    Child { parent: usize },
}

#[derive(Debug, Default)]
struct UF(Vec<UFNode>);

struct Root {
    idx: usize,
    size: usize,
    and: i32,
}
impl UF {
    fn init(&mut self, n: usize) {
        self.0 = vec![UFNode::Root { size: 1, and: -1 }; n];
    }

    fn parent(&mut self, idx: usize) -> Root {
        match self.0[idx] {
            UFNode::Root { size, and } => Root { idx, size, and },
            UFNode::Child { parent } => {
                let res = self.parent(parent);
                if res.idx != parent {
                    self.0[idx] = UFNode::Child { parent: res.idx };
                }
                res
            }
        }
    }

    fn merge(&mut self, i: usize, j: usize, w: i32) {
        let Root { idx: mut ri, and: ai, size: si } = self.parent(i);
        let Root { idx: mut rj, and: aj, size: sj } = self.parent(j);
        if ri == rj {
            if let UFNode::Root { and, .. } = &mut self.0[ri] {
                *and &= w;
            }
            return;
        }
        if si < sj {
            [ri, rj] = [rj, ri];
        }
        self.0[ri] = UFNode::Root { size: si + sj, and: ai & aj & w };
        self.0[rj] = UFNode::Child { parent: ri };
    }

    fn query(&mut self, i: usize, j: usize) -> i32 {
        let ri = self.parent(i);
        let rj = self.parent(j);
        if ri.idx != rj.idx {
            -1
        } else {
            ri.and
        }
    }
}

pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
    let mut uf = UF::default();
    uf.init(n as usize);
    for e in &edges {
        uf.merge(e[0] as usize, e[1] as usize, e[2]);
    }
    query.iter().map(|q| uf.query(q[0] as usize, q[1] as usize)).collect()
}
