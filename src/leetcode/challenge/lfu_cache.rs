use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, PartialOrd, Default, Clone, Copy, Debug)]
struct Usage {
    count: u32,
    index: u32,
}

#[derive(Default)]
struct LFUC<K, V> {
    data: HashMap<K, (V, Usage)>,
    usage: BTreeMap<Usage, K>,
    index: u32,
    capacity: usize,
}

impl<K: Eq + Hash + Clone, V> LFUC<K, V> {
    fn new(capacity: i32) -> Self
    where
        K: Default,
        V: Default,
    {
        let capacity = capacity as usize;
        Self {
            capacity,
            ..Default::default()
        }
    }

    fn try_get(&mut self, key: &K) -> Option<&mut V> {
        self.index += 1;
        let (v, u) = self.data.get_mut(&key)?;
        if let Some(k) = self.usage.remove(u) {
            u.count += 1;
            u.index = self.index;
            self.usage.insert(*u, k);
        }
        Some(v)
    }

    fn remove_worst(&mut self) {
        if let Some((&u, k)) = self.usage.iter().next() {
            let k = k.clone();
            self.usage.remove(&u);
            self.data.remove(&k);
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.capacity == 0 {
            return
        }
        if let Some(v) = self.try_get(&key) {
            *v = value;
            return;
        }
        if self.data.len() == self.capacity {
            self.remove_worst();
        }
        let usage = Usage {
            count: 1,
            index: self.index,
        };
        self.data.insert(key.clone(), (value, usage));
        self.usage.insert(usage, key);
    }
}

impl LFUC<i32, i32> {
    fn get(&mut self, key: i32) -> i32 {
        self.try_get(&key).copied().unwrap_or(-1)
    }
}

type LFUCache = LFUC<i32, i32>;
