use crate::data::leetcode::Tree;
use std::mem::swap;

pub fn pseudo_palindromic_paths(root: Tree) -> i32 {
    pali_traverse(root, 0)
}

fn pali_traverse(root: Tree, code: u16) -> i32 {
    println!("{:?} {:b}", root, code);
    let rc = if let Some(rc) = root { rc } else { return 0; };
    let tn = rc.borrow();
    let new_code = code ^ (1 << tn.val);
    if tn.left.is_none() && tn.right.is_none() {
        return if ((new_code - 1) & new_code) == 0 { 1 } else { 0 };
    }
    pali_traverse(tn.left.clone(), new_code) + pali_traverse(tn.right.clone(), new_code)
}

pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let n = board.len();
    let m = board.first().map(|v| v.len()).unwrap_or(0);
    let mut new = vec![vec![0; m]; n];
    let nrange = |x: usize, b: usize| x.min(x.max(1) - 1)..(x + 2).min(b);
    let neighbors = |i: usize, j: usize| nrange(i, n).flat_map(move |x| nrange(j, m).map(move |y| (x, y)));
    for i in 0..n {
        for j in 0..m {
            let s: i32 = neighbors(i, j).map(|(x, y)| board[x][y]).sum();
            if board[i][j] == 1 && (s == 3 || s == 4) || board[i][j] == 0 && s == 3 {
                new[i][j] = 1;
            }
        }
    }
    swap(board, &mut new);
}

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    if n == 0 { return 0 }
    let mut fen = vec![0; n * 4];
    fill_fen(&heights, &mut fen, |x, y| x.min(y));

    heights.into_iter().enumerate()
        .map(|(i, v)| count_not_less(n, &fen, i, v) as i32 * v)
        .max().unwrap_or(0)
}

fn fill_fen<A: Copy, F: Copy>(src: &[A], res: &mut [A], f: F) where F: Fn(A, A) -> A {
    fill_fen_iter(src, res, f, 0, 0, src.len());
}


fn fill_fen_iter<A: Copy, F: Copy>(src: &[A], res: &mut [A], f: F, p: usize, from: usize, to: usize) -> A where F: Fn(A, A) -> A {
    res[p] = if to - from == 1 {
        src[from]
    } else {
        let m = (from + to) / 2;
        let l = fill_fen_iter(src, res, f, 2 * p + 1, from, m);
        let r = fill_fen_iter(src, res, f, 2 * p + 2, m, to);
        f(l, r)
    };
    res[p]
}

fn count_not_less(n: usize, fen: &[i32], i: usize, v: i32) -> usize {
    count_not_less_iter(fen, i, v, 0, 0, n).map(|(l, r)| r - l).unwrap_or(0)
}

fn count_not_less_iter(fen: &[i32], i: usize, v: i32, p: usize, from: usize, to: usize) -> Option<(usize, usize)> {
    if fen[p] >= v { return Some((from, to)); };
    if to - from == 1 { return None; }
    let m = (from + to) / 2;
    return if i >= m {
        let (rl, rr) = count_not_less_iter(fen, i, v, 2 * p + 2, m, to)?;
        if rl == m {
            let ll = count_not_less_iter(fen, i, v, 2 * p + 1, from, m).map(|(l, _)| l).unwrap_or(rl);
            Some((ll, rr))
        } else { Some((rl, rr)) }
    } else {
        let (ll, lr) = count_not_less_iter(fen, i, v, 2 * p + 1, from, m)?;
        if lr == m {
            let rr = count_not_less_iter(fen, i, v, 2 * p + 2, m, to).map(|(_, r)| r).unwrap_or(lr);
            Some((ll, rr))
        } else { Some((ll, lr)) }
    };
}


#[test]
fn largest_lol() {
    fn check(x: &[i32], exp: i32) { assert_eq!(largest_rectangle_area(x.iter().copied().collect()), exp) }
    check(&[2, 1, 5, 6, 2, 3], 10);
    check(&[0, 1, 2, 3, 4, 5, 6, 7, 8], 20);
    check(&[], 0);
}