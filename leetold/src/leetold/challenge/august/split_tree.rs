use data::leetcode::Tree;

const MOD: i64 = 1000_000_007;
pub fn max_product(root: Tree) -> i32 {
    let mut m = MaxProduct::default();
    m.sum(&root);
    m.prod(&root);
    (m.prod % MOD) as i32
}

#[derive(Default)]
struct MaxProduct {
    sum: i64,
    pub prod: i64,
}
impl MaxProduct {
    pub fn sum(&mut self, node: &Tree) -> Option<()> {
        let n = node.as_ref()?.borrow();
        self.sum += n.val as i64;
        self.sum(&n.left);
        self.sum(&n.right)
    }
    pub fn split(&mut self, s: i64){
        self.prod = self.prod.max((self.sum - s) * s)
    }
    pub fn prod(&mut self, node: &Tree) -> Option<i64>{
        let n = node.as_ref()?.borrow();
        let mut total = n.val as i64;
        if let Some(l) = self.prod(&n.left){
            total += l;
            self.split(l)
        }
        if let Some(r) = self.prod(&n.right){
            total += r;
            self.split(r)
        }
        Some(total)
    }
}
