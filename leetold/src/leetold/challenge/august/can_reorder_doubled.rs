use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Index, IndexMut},
};

#[derive(Default)]
struct Counter<A>(HashMap<A, i32>);
impl<A: Eq + Hash> Index<A> for Counter<A> {
    type Output = i32;

    fn index(&self, index: A) -> &Self::Output {
        self.0.get(&index).unwrap_or(&0)
    }
}

impl<A: Eq + Hash> IndexMut<A> for Counter<A> {
    fn index_mut(&mut self, index: A) -> &mut Self::Output {
        self.0.entry(index).or_insert(0)
    }
}
pub fn can_reorder_doubled(mut arr: Vec<i32>) -> bool {
    arr.sort_by_key(|v| -v.abs());
    let mut counter: Counter<i32> = Counter::default();
    for &i in &arr {
        counter[i] += 1;
    }
    for i in arr {
        if counter[i] == 0 {
            continue;
        }
        counter[i] -= 1;
        if i % 2 != 0 || counter[i / 2] == 0 {
            return false;
        }
        counter[i / 2] -= 1;
    }
    true
}
