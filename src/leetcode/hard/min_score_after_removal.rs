use std::ops::{BitXor, Sub};

pub fn minimum_score<A: AsRef<[i32]>>(nums: Vec<i32>, edges: Vec<A>) -> i32 {
    let edges = edges.iter().map(|v| [v.as_ref()[0] as usize, v.as_ref()[1] as usize]);
    Tree::new(nums, edges).score()
}

struct Tree<A> {
    value: A,
    children: Vec<Tree<A>>,
}

struct Node<A> {
    start: u32,
    end: u32,
    xor: A,
}

impl<A: Copy + Default + Ord + BitXor<Output = A> + Sub<Output = A>> Tree<A> {
    fn new(nums: Vec<A>, edges: impl IntoIterator<Item = [usize; 2]>) -> Self {
        let mut adj: Vec<_> = nums.into_iter().map(|x| (x, vec![])).collect();
        for [i, j] in edges {
            adj[i].1.push(j);
            adj[j].1.push(i);
        }
        Self::build(&adj, 0, None)
    }

    fn build(adj: &[(A, Vec<usize>)], i: usize, prev: Option<usize>) -> Self {
        let value = adj[i].0;
        let build = |&j| (Some(j) != prev).then(|| Self::build(adj, j, Some(i)));
        let children = adj[i].1.iter().filter_map(build).collect();
        Tree { value, children }
    }

    fn calc_of(x: A, y: A, z: A) -> A {
        let mx = x.max(y).max(z);
        let mn = x.min(y).min(z);
        mx - mn
    }

    fn node_calc(n1: &Node<A>, n2: &Node<A>, gx: A) -> A {
        if n2.start <= n1.start && n2.end >= n1.end {
            Self::calc_of(n1.xor, n2.xor ^ n1.xor, gx ^ n2.xor)
        } else {
            Self::calc_of(n1.xor, n2.xor, gx ^ n1.xor ^ n2.xor)
        }
    }

    fn score(&self) -> A {
        let mut nodes = vec![];
        let mut global_xor = A::default();
        self.traverse(&mut nodes, &mut 0, &mut global_xor);
        let n = nodes.len() - 1;
        let nodes = &nodes;
        (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| Self::node_calc(&nodes[i], &nodes[j], global_xor)))
            .min()
            .unwrap_or(A::default())
    }

    fn traverse(&self, nodes: &mut Vec<Node<A>>, counter: &mut u32, acc: &mut A) {
        *counter += 1;
        let before = *acc;
        let start = *counter;
        for c in &self.children {
            c.traverse(nodes, counter, acc);
        }
        let end = *counter;
        *acc = *acc ^ self.value;
        let xor = *acc ^ before;
        nodes.push(Node { start, end, xor })
    }
}

#[test]
fn test2() {
    assert_eq!(
        0,
        minimum_score(vec![5, 5, 2, 4, 4, 2], vec![[0, 1], [1, 2], [5, 2], [4, 3], [1, 3]])
    )
}

#[test]
fn test1() {
    assert_eq!(
        9,
        minimum_score(vec![1, 5, 5, 4, 11], vec![[0, 1], [1, 2], [1, 3], [3, 4]])
    )
}

#[test]
fn wa1() {
    assert_eq!(11, minimum_score(vec![9, 14, 2, 1], vec![[2, 3], [3, 0], [3, 1]]))
}
