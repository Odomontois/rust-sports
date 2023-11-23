use std::collections::{HashMap, VecDeque};

struct LRUCache {
    q: VecDeque<(i32, usize)>,
    keys: HashMap<i32, (i32, usize)>,
    cap: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let cap = capacity as usize;
        Self {
            q: VecDeque::new(),
            keys: HashMap::new(),
            cap,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some((v, cnt)) = self.keys.get_mut(&key) {
            *cnt += 1;
            self.q.push_back((key, *cnt));
            *v
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if !self.keys.contains_key(&key) && self.keys.len() == self.cap {
            while let Some((k, cnt)) = self.q.pop_front() {
                if cnt == self.keys[&k].1 {
                    self.keys.remove(&k);
                    break;
                }
            }
        }

        self.q.push_back((key, 0));
        let (val, cnt) = self.keys.entry(key).or_insert((value, 0));
		*val = value;
		*cnt += 1;
    }
}

#[test]
fn example() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}
