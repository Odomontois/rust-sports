#[derive(Debug)]
struct Tree { children: Vec<Vec<usize>>, root: usize }

trait DFS<S = ()> {
    fn go_down(parent: usize, child: usize, pval: &mut Self, s: S) -> Self;
    fn go_up(parent: usize, child: usize, pval: &mut Self, cval: &Self, s: S);
}

impl FromIterator<(usize, usize)> for Tree {
    fn from_iter<T: IntoIterator<Item=(usize, usize)>>(iter: T) -> Self {
        let mut v = vec![Vec::<usize>::new()];
        let mut root = 0;
        iter.into_iter().for_each(|(par, child)| {
            while v.len() <= par.max(child) { v.push(Vec::new()) }
            root = par;
            v[par].push(child);
            v[child].push(par)
        });
        let mut seen = HashSet::new();
        seen.insert(root);
        let mut stack = vec![(root, 0usize)];
        let mut children = vec![Vec::new(); v.len()];
        while let Some((par, len)) = stack.pop() {
            if len < v[par].len() {
                let child = v[par][len];
                stack.push((par, len + 1));
                if !seen.contains(&child) {
                    children[par].push(child);
                    stack.push((child, 0));
                    seen.insert(child);
                }
            }
        }

        Tree { children, root }
    }
}

impl Tree {
    fn dfs_<A: DFS + Clone>(&self, root_val: A) -> Vec<A> { self.dfs(root_val, ()) }

    fn dfs<A: DFS<S> + Clone, S: Clone>(&self, root_val: A, s: S) -> Vec<A> {
        let mut vals = vec![root_val.clone(); self.children.len()];
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut cur = self.root;
        let mut cur_val = root_val;
        let mut len = 0usize;
        let mut finish = false;
        while !finish {
            if len >= self.children[cur].len() {
                if let Some((par, par_len)) = stack.pop() {
                    DFS::go_up(par, cur, &mut vals[par], &cur_val, s.clone());
                    cur = par;
                    cur_val = vals[par].clone();
                    len = par_len + 1;
                } else { finish = true }
            } else {
                let child = self.children[cur][len];
                vals[child] = DFS::go_down(cur, child, &mut vals[cur], s.clone());
                cur_val = vals[child].clone();
                stack.push((cur, len));
                cur = child;
                len = 0;
            }
        }
        vals
    }
}

use std::convert::TryFrom;
use std::iter::*;
use std::collections::HashSet;


impl DFS for () {
    fn go_down(parent: usize, child: usize, _: &mut Self, _: ()) -> Self {
        println!("down {} {}", parent, child)
    }

    fn go_up(parent: usize, child: usize, _: &mut Self, _: &Self, _: ()) {
        println!("up {} {}", parent, child)
    }
}

type ChildLength = (usize, i32);

#[derive(Copy, Clone, Debug)]
struct Top2 { first: Option<ChildLength>, second: Option<ChildLength> }

impl Top2 {
    fn new() -> Top2 { Top2 { first: None, second: None } }
    fn update(&mut self, i: usize, l: i32) {
        let upd = Some((i, l));
        if let Some((_, fl)) = self.first {
            if fl <= l {
                self.second = self.first;
                self.first = upd;
            } else if let Some((_, sl)) = self.second {
                if sl <= l { self.second = upd }
            } else { self.second = upd }
        } else { self.first = upd };
    }
}

impl DFS for Top2 {
    fn go_down(_: usize, _: usize, _: &mut Self, _: ()) -> Self { Top2::new() }

    fn go_up(_: usize, child: usize, pval: &mut Self, cval: &Self, _: ()) {
        let clengh = cval.first.map(|(_, x)| x + 1).unwrap_or(1);
        pval.update(child, clengh);
    }
}


impl DFS<&Vec<Top2>> for Top2 {
    #[allow(dead_code)]
    fn go_down(parent: usize, child: usize, pval: &mut Self, ts: &Vec<Top2>) -> Self {
        let Top2 { first, second } = pval.clone();
        let pchain =
            first.into_iter().chain(second.into_iter()).find(|&(i, _)| i != child)
                .map(|(_, l)| l + 1).unwrap_or(1);

        let mut top = ts[child].clone();
        top.update(parent, pchain);
        top
    }

    fn go_up(_: usize, _: usize, _: &mut Self, _: &Self, _: &Vec<Top2>) {}
}

#[allow(dead_code)]
pub fn find_min_height_trees_impl<A: IntoIterator<Item=[i32; 2]>>(edges: A) -> Vec<i32> {
    let tree: Tree = edges.into_iter().map(|[x, y]| (x as usize, y as usize)).collect();
    let tops = tree.dfs_(Top2::new());
    let lengths: Vec<_> =
        tree.dfs(tops[tree.root], &tops).iter().filter_map(|l2| l2.first.map(|x| x.1)).enumerate().collect();
    let best = lengths.iter().map(|&x| x.1).min().unwrap_or(0);
    if lengths.is_empty() { vec![0] } else { lengths.iter().filter_map(|&(i, l)| if l == best { Some(i as i32) } else { None }).collect() }
}

#[allow(dead_code)]
pub fn find_min_height_trees(_: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    find_min_height_trees_impl(edges.iter().map(|x| x.as_slice()).flat_map(<&[i32; 2]>::try_from).cloned())
}