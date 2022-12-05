use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem::take;

use rand::rngs::ThreadRng;
use rand::Rng;

struct Element<A> {
    val: A,
    next: u32,
    removed: bool,
}

#[derive(Default)]
struct RandomSet<A> {
    table: Vec<u32>,
    elements: Vec<Element<A>>,
    size: i64,
    rng: ThreadRng,
}

#[derive(Clone, Copy)]
enum Index {
    Table(usize),
    Elem(usize),
}

impl<A: Hash + Eq> RandomSet<A> {
    fn new() -> Self
    where
        A: Default,
    {
        Self::default()
    }

    fn ptr(&mut self, index: Index) -> &mut u32 {
        match index {
            Index::Table(i) => &mut self.table[i],
            Index::Elem(i) => &mut self.elements[i].next,
        }
    }

    fn table_index(&self, val: &A) -> Index {
        let mut s = DefaultHasher::new();
        val.hash(&mut s);
        Index::Table(s.finish() as usize % self.table.len())
    }

    fn elem_at(&mut self, index: Index) -> usize {
        *self.ptr(index) as usize - 1
    }

    fn search(&mut self, val: &A) -> (bool, Index) {
        let mut ix = self.table_index(&val);
        while *self.ptr(ix) != 0 {
            let i = self.elem_at(ix);
            if self.elements[i].val == *val {
                return (true, ix);
            }
            ix = Index::Elem(i);
        }
        return (false, ix);
    }

    fn resize(&mut self, delta: i64) {
        self.size += delta;
        let tlen = self.table.len() as i64;
        let elen = self.elements.len() as i64;
        if (tlen..tlen * 4).contains(&self.size) && self.size * 2 > elen {
            return;
        }
        let old = take(&mut self.elements);
        self.table = vec![0; 2 * self.size as usize];
        for Element { val, removed, .. } in old {
            if !removed {
                self.ins(val);
            }
        }
    }

    fn ins(&mut self, val: A) -> bool {
        let (found, ix) = self.search(&val);
        if !found {
            *self.ptr(ix) = self.elements.len() as u32 + 1;
            self.elements.push(Element {
                val,
                next: 0,
                removed: false,
            });
        }
        found
    }

    fn insert(&mut self, val: A) -> bool {
        if self.table.is_empty() {
            self.table.push(0);
        }
        let found = self.ins(val);
        if !found {
            self.resize(1);
        }
        !found
    }

    fn remove(&mut self, val: A) -> bool {
        if self.table.is_empty() {
            return false;
        }
        let (found, ix) = self.search(&val);
        if found {
            let i = self.elem_at(ix);
            self.elements[i].removed = true;
            *self.ptr(ix) = self.elements[i].next;
            self.resize(-1);
        }
        found
    }

    fn get_random(&mut self) -> A
    where
        A: Copy,
    {
        let mut i = self.rng.gen_range(0, self.elements.len());
        while self.elements[i].removed {
            i = self.rng.gen_range(0, self.elements.len());
        }
        self.elements[i].val
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

#[test]
fn test2() {
    let mut set = RandomizedSet::new();
    assert!(set.insert(0));
    assert!(set.insert(2));
    assert!(set.insert(1));
    assert!(!set.insert(1));
    assert!(!set.insert(0));
    assert!(set.remove(0));
    assert!([1, 2].contains(&set.get_random()));
    assert!(!set.insert(1));
    assert!(set.remove(2));
}
