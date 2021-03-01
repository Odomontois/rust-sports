use std::{
    cmp::{Ordering, PartialOrd},
    usize,
};

#[derive(PartialEq, Debug, Clone, Copy)]
struct Bi<A, B>(A, B);

impl<A: PartialOrd, B: PartialOrd> PartialOrd for Bi<A, B> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.0 <= other.0 && self.1 <= other.1 {
            Some(Ordering::Less)
        } else if self.0 >= other.0 && self.1 >= other.1 {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

type Cost = Bi<i32, i32>;

fn bin_search_with<A, B: Ord, F>(xs: &[A], x: &B, f: F) -> usize
where
    F: Fn(&A) -> B,
{
    xs.binary_search_by_key(x, f).unwrap_or_else(|x| x)
}

pub fn drop_trailing_gt<I>(xs: I) -> Vec<I::Item>
where
    I: IntoIterator,
    I::IntoIter: DoubleEndedIterator,
    I::Item: PartialOrd + Copy,
{
    let mut res: Vec<_> = xs
        .into_iter()
        .rev()
        .scan(None, |prev, x| {
            Some(if prev.iter().any(|y| &x > y) {
                None
            } else {
                *prev = Some(x);
                Some(x)
            })
        })
        .flatten()
        .collect();
    res.reverse();
    res
}

pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
    arr2.sort();
    arr1.into_iter()
        .fold(vec![Bi(-1, 0)], |state, n| {
            let i = bin_search_with(&state, &n, |x| x.0);
            let keep = if i > 0 { Some(Bi(n, state[i - 1].1)) } else { None };
            let search_next = |Bi(x, c)| {
                Some(bin_search_with(&arr2, &(x + 1), |&x| x))
                    .filter(|&j| j < arr2.len())
                    .map(|j| Bi(arr2[j], c + 1))
            };

            let mut state: Vec<_> = drop_trailing_gt(
                state
                    .into_iter()
                    .filter_map(search_next)
                    .filter(|x| !keep.iter().any(|y| x >= y)),
            );

            let keep = keep.filter(|x| !state.iter().any(|y| x >= y));
            if let Some(k) = keep {
                let place = bin_search_with(&state, &k.0, |&Bi(x, _)| x);
                state.insert(place, k)
            }

            state
        })
        .last()
        .copied()
        .map(|Bi(_, c)| c)
        .unwrap_or(-1)
}

#[test]
fn check_make() {
    assert_eq!(make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]), 1);
    assert_eq!(make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1]), 2);
    assert_eq!(make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3]), -1);
    assert_eq!(
        make_array_increasing(
            vec![23, 10, 9, 12, 3, 14, 21, 16, 7, 10, 17, 12],
            vec![6, 5, 0, 15, 2, 17, 4, 11, 6, 5, 8, 15, 10, 1, 20, 11, 14, 13, 8],
        ),
        11
    );
}
