use std::{collections::HashMap, hash::Hash};

use rand::{thread_rng, Rng};

#[derive(Default)]
struct RandomSet<A> {
    elems: HashMap<A, usize>,
    choose: Vec<A>,
}

impl<A: Hash + Eq + Copy> RandomSet<A> {
    fn new() -> Self
    where
        A: Default,
    {
        Self::default()
    }

    fn insert(&mut self, val: A) -> bool {
        if self.elems.contains_key(&val) {
            return false;
        }
        self.elems.insert(val, self.choose.len());
        self.choose.push(val);
        true
    }

    fn remove(&mut self, val: A) -> bool {
        if let Some(ix) = self.elems.remove(&val) {
            let last = *self.choose.last().unwrap();
            if last != val {
                self.elems.insert(last, ix);
            }
            self.choose[ix] = last;
            self.choose.pop();
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> A {
        let i = thread_rng().gen_range(0, self.choose.len());
        self.choose[i]
    }
}

type RandomizedSet = RandomSet<i32>;

#[test]
fn test1() {
    let mut set = RandomizedSet::new();
    assert!(set.insert(1));
    assert!(!set.remove(2));
    assert!(set.insert(2));
    assert!([1, 2].contains(&set.get_random()));
    assert!(set.remove(1));
    assert!(!set.insert(2));
    assert_eq!(2, set.get_random());
}
