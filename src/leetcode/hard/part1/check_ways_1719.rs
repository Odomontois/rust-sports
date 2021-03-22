use std::collections::{HashMap, HashSet};

pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
    let mut h = HashMap::<_, Vec<_>>::new();
    let mut add = |i: usize, j: usize| if let Some(v) = h.get_mut(&i) {
        v.push(j)
    } else {
        h.insert(i, vec![j]);
    };
    for v in pairs {
        add(v[0] as usize, v[1] as usize);
        add(v[1] as usize, v[0] as usize);
    }
    ways(h).map(|many| if many { 2 } else { 1 }).unwrap_or(0)
}

fn ways(g: HashMap<usize, Vec<usize>>) -> Option<bool> {
    let mut roots = g.iter().filter_map(|(&k, v)|
        Some(k).filter(|_| v.len() + 1 == g.len())
    );
    let root = roots.next()?;
    let many = roots.next().is_some();
    let mut seen = HashSet::new();
    let mut stack = Vec::new();
    let mut comps = Vec::new();
    for &i in g.keys() {
        if !seen.insert(i) { continue; };
        let mut comp = Vec::new();
        stack.push(i);
        while let Some(j) = stack.pop() {
            comp.push(j);
            for k in &g[&j] {}
        }
        comps.push(comp);
    }
    unimplemented!()
}