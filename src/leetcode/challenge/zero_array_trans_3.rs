pub fn max_removal(nums: Vec<i32>, queries: Vec<impl std::ops::Index<usize, Output = i32>>) -> i32 {
    use std::{array::from_fn, collections::BinaryHeap};

    let [mut s, mut e] = from_fn(|_| BinaryHeap::<i32>::new());
    let mut qs: Vec<_> = queries.iter().map(|v| [v[0], v[1] + 1]).collect();
    qs.sort_unstable_by_key(|v| v[0]);
    let mut qsi = qs.into_iter().peekable();
    let mut cur = 0;
    let mut res = queries.len() as i32;
    for (i, x) in nums.into_iter().enumerate() {
        let i = i as i32;
        while e.peek().filter(|&x| -x == i).is_some() {
            cur -= 1;
            e.pop();
        }
        while let Some([_, r]) = qsi.next_if(|v| v[0] == i) {
            s.push(r);
        }
        for _ in 0..(x - cur) {
            if let Some(r) = s.pop().filter(|&y| y > i) {
                cur += 1;
                res -= 1;
                e.push(-r);
            } else {
                return -1;
            }
        }
    }
    res
}

#[test]
fn hh() {
    println!("{}", std::mem::size_of::<Vec<i32>>());
    println!("{}", std::mem::size_of::<[usize; 3]>());
}

#[test]
fn example1() {
    assert_eq!(1, max_removal(vec![2, 0, 2], vec![[0, 2], [0, 2], [1, 1]]));
}

#[test]
fn example2() {
    assert_eq!(2, max_removal(vec![1, 1, 1, 1], vec![[1, 3], [0, 2], [1, 3], [1, 2]]));
}

#[test]
fn example3() {
    assert_eq!(-1, max_removal(vec![1, 2, 3, 4], vec![[0, 3]]));
}

#[test]
fn wa() {
    assert_eq!(2, max_removal(vec![0, 3], vec![[0, 1], [0, 0], [0, 1], [0, 1], [0, 0]]));
}

#[test]
fn wa1() {
    assert_eq!(
        -1,
        max_removal(
            vec![0, 0, 4],
            vec![[1, 1], [1, 1], [1, 2], [0, 0], [1, 1], [1, 1], [1, 2]]
        )
    );
}
