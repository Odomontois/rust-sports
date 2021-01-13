#[derive(Clone, Copy, Debug)]
enum DU { Parent(usize), Size(usize) }

#[derive(Clone, Debug)]
pub struct DisjointUnion { du: Vec<DU> }

impl DisjointUnion {
    pub fn new(size: usize) -> Self { DisjointUnion { du: vec![DU::Size(1); size] } }

    pub fn root(&mut self, i: usize) -> (usize, usize) {
        match self.du[i] {
            DU::Parent(p) => {
                let (r, s) = self.root(p);
                if p != r { self.du[i] = DU::Parent(r) }
                (r, s)
            }
            DU::Size(s) => (i, s)
        }
    }

    pub fn joined(&mut self, i: usize, j: usize) -> bool {
        self.root(i).0 == self.root(j).0
    }

    pub fn join(&mut self, i: usize, j: usize) {
        let (ir, is) = self.root(i);
        let (jr, js) = self.root(j);
        if ir == jr { return; }
        let (win, loose) = if is < js { (jr, ir) } else { (ir, jr) };
        self.du[loose] = DU::Parent(win);
        self.du[win] = DU::Size(is + js);
    }
}