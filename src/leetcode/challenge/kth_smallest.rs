use std::{
    cmp::{
        Ordering::{Greater, Less},
        Reverse,
    },
    collections::BinaryHeap,
};

use data::ignore::Ign;

pub fn kth_smallest2(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut h = BinaryHeap::new();
    for row in &matrix {
        h.push((Reverse(row[0]), Ign(&row[1..])));
    }
    for _ in 1..k {
        if let Some((_, Ign(v))) = h.pop() {
            if !v.is_empty() {
                h.push((Reverse(v[0]), Ign(&v[1..])));
            }
        }
    }

    if let Some((Reverse(x), _)) = h.pop() {
        return x;
    }

    -1337
}

pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let k = k as usize;
    let elems = matrix.iter().flatten().copied();
    let mut min = elems.clone().min().unwrap_or(0) - 1;
    let mut max = elems.max().unwrap_or(0) ;
    while max - min > 1 {
        let m = (max + min) / 2;
        let u: usize = matrix
            .iter()
            .map(|v| v.binary_search_by(|y| if y <= &m { Less } else { Greater }).unwrap_err())
            .sum();
        if u < k {
            min = m;
        } else {
            max = m;
        }
    }

    max
}
