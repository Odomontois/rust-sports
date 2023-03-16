use std::hash::Hash;

pub fn dup_pref<'a, A, B, I>(elems: impl IntoIterator<Item = A>, f: impl Fn(A) -> B) -> Vec<(usize, Vec<usize>)>
where
    B: IntoIterator<Item = I>,
    I: Eq + Hash,
{
    let iterators: Vec<_> = elems
        .into_iter()
        .enumerate()
        .map(|(i, a)| (i, f(a).into_iter().peekable()))
        .collect();

    let mut buckets = vec![iterators];
    let mut res = vec![];
    while !buckets.is_empty() {
        let mut new_buckets = vec![];
        for mut bucket in buckets {
            let mut is = std::collections::HashMap::new();
            if let Some(i) = bucket.iter_mut().find_map(|(i, v)| v.peek().is_none().then(|| *i)) {
                res.push((i, bucket.into_iter().filter_map(|(j, _)| (i != j).then(|| j)).collect()));
                continue;
            }
            for (i, mut iter) in bucket {
                if let Some(next) = iter.next() {
                    is.entry(next).or_insert(vec![]).push((i, iter));
                }
            }
            new_buckets.extend(is.into_values().filter(|v| v.len() > 1));
        }
        buckets = new_buckets;
    }
    res
}

#[test]
fn check() {
    assert_eq!(
        dup_pref(["a", "a.b", "a.b.c", "c", "d.c"], |s| s.as_bytes()),
        vec![(0, vec![1, 2])]
    );

    assert_eq!(
        dup_pref(["a", "a.b", "a.b.c", "c", "c.d"], |s| s.as_bytes()),
        vec![(0, vec![1, 2]), (3, vec![4])]
    );
}
