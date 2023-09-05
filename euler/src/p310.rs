struct SquareNim {
    size: usize,
}

impl SquareNim {
    fn pile_numbers(&self) -> Vec<usize> {
        let mut res = Vec::with_capacity(self.size + 1);
        for c in 0..=self.size {
            let squares = (1..).map(|i| i * i).take_while(|&q| c >= q);
            let not_seen = |&x: &usize| squares.clone().all(|q| res[c - q] != x);
            res.push((0..).filter(not_seen).next().unwrap())
        }
        res
    }

    fn pile_counts(&self) -> Vec<usize> {
        let mut res = Vec::new();
        for i in self.pile_numbers() {
            add(&mut res, i, 1);
        }
        res
    }

    fn counts(&self) -> Vec<usize> {
        let pc = self.pile_counts();
        let mut res = vec![1];
        for _ in 0..3 {
            let mut next = Vec::new();
            for (i, x) in pc.iter().enumerate() {
                for (j, y) in res.iter().enumerate() {
                    add(&mut next, i ^ j, x * y);
                }
            }
            res = next;
        }
        res
    }

    fn chpok(&self) -> usize {
        let pile = self.pile_counts()[0];
        let tri = self.counts()[0];
        let triple = pile;
        let double = pile * self.size;
        let diff = tri - triple - double * 3;
        diff / 6 + double + triple
    }
}

fn add(v: &mut Vec<usize>, i: usize, d: usize) {
    while v.len() <= i {
        v.push(0);
    }
    v[i] += d;
}

fn with_index<A>(v: Vec<A>) -> Vec<(usize, A)> {
    v.into_iter().enumerate().collect()
}

#[test]
fn runn() {
    let g = SquareNim { size: 29 };
    println!(
        "size = {},  {:?}\
        \nfor single pile {:?}\
        \nres = {}",
        g.size,
        with_index(g.pile_counts()),
        with_index(g.counts()),
        g.chpok()
    );
}
