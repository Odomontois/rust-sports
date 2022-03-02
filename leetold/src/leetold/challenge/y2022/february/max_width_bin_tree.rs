use data::leetcode::Tree;

pub fn width_of_binary_tree(root: Tree) -> i32 {
    let depths = &mut vec![];
    count(&root, depths, 0, 0);
    depths.iter().map(|&[first, last]| last - first + 1).max().unwrap_or(0) as i32
}

fn count(root: &Tree, acc: &mut Vec<[usize; 2]>, depth: usize, idx: usize) {
    let node = if let Some(b) = root {
        b.borrow()
    } else {
        return;
    };
    if depth == acc.len() {
        acc.push([idx, idx]);
    } else {
        acc[depth][1] = idx;
    }
    count(&node.left, acc, depth + 1, 2 * idx + 1);
    count(&node.right, acc, depth + 1, 2 * idx + 2);
}
