use std::collections::HashMap;

pub fn num_submatrix_sum_target<A: AsRef<[i32]>>(matrix: Vec<A>, target: i32) -> i32 {
    let (n, m) = (matrix.len(), matrix[0].as_ref().len());
    let mut res = 0;
    for s in 0..n {
        for e in s..n {
            let mut elems = HashMap::new();
            let mut x: i32 = 0;
            for j in 0..m {
                *elems.entry(x).or_insert(0) += 1;
                x += (s..=e).map(|i| matrix[i].as_ref()[j]).sum::<i32>();
                res += elems.get(&(x - target)).copied().unwrap_or(0);
            }
        }
    }
    res
}

#[test]
fn example1() {
    assert_eq!(4, num_submatrix_sum_target(vec![[0, 1, 0], [1, 1, 1], [0, 1, 0]], 0))
}
