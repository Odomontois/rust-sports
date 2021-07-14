use std::collections::BTreeMap;

use crate::data::leetcode::Tree;

pub fn vertical_traversal(root: Tree) -> Vec<Vec<i32>> {
    let mut vs = BTreeMap::new();
    v_traverse(root, 0, 0, &mut vs);
    vs.into_iter().map(|(_, mut v)| {
        v.sort();
        v.into_iter().map(|(_, v)| v).collect()
    }).collect()
}

fn v_traverse(root: Tree, x: i32, y: i32, vs: &mut BTreeMap<i32, Vec<(i32, i32)>>) -> Option<()> {
    let v = root?;
    let r = v.borrow();
    vs.entry(x).or_insert(vec![]).push((y, r.val));
    v_traverse(r.left.clone(), x - 1, y + 1, vs);
    v_traverse(r.right.clone(), x + 1, y + 1, vs);
    Some(())
}

pub fn next_permutation<A: Ord>(nums: &mut Vec<A>) {
    let swaps = (0..nums.len() - 1).rev()
        .find(|&i| nums[i] < nums[i + 1])
        .and_then(|i|
            (0..nums.len()).rev()
                .skip_while(|&j| nums[i] >= nums[j])
                .next().map(|j| (i, j)));
    if let Some((i, j)) = swaps {
        nums.swap(i, j);
        nums[i + 1..].reverse();
    } else {
        nums.reverse();
    }
}

#[test]
fn check_perm() {
    fn check(xs: &[i32], exp: &[i32]) {
        let mut xs = xs.iter().copied().collect();
        next_permutation(&mut xs);
        assert_eq!(&xs[..], exp);
    }
    check(&[1, 2, 3], &[1, 3, 2]);
    check(&[1, 5, 1], &[5, 1, 1]);
}

