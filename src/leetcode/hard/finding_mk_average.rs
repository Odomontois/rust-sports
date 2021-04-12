use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Default, Clone)]
struct MSet {
    elems: BTreeMap<i32, usize>,
    sum: i64,
    size: usize,
    lim: usize,
    top: bool,
}

impl MSet {
    fn new(lim: usize, top: bool) -> Self {
        Self {
            lim,
            top,
            ..Self::default()
        }
    }
    fn remove(&mut self, x: i32) -> bool {
        let c = if let Some(x) = self.elems.get_mut(&x) {
            *x -= 1;
            *x
        } else {
            return false;
        };
        if c == 0 {
            self.elems.remove(&x);
        }
        self.sum -= x as i64;
        self.size -= 1;
        true
    }

    fn remove_and_fill(&mut self, x: i32, fill: &mut MSet) -> bool {
        if !self.remove(x) {
            return false;
        }
        let mut it = fill.elems.keys();
        let &x = (if self.top { it.next_back() } else { it.next() }).unwrap();
        self.put(x);
        fill.remove(x)
    }

    fn put(&mut self, x: i32) -> Option<i32> {
        *self.elems.entry(x).or_insert(0) += 1;
        self.sum += x as i64;
        self.size += 1;
        if self.size <= self.lim {
            return None;
        }
        let mut it = self.elems.keys();
        let &next = (if self.top { it.next() } else { it.next_back() }).unwrap();
        self.remove(next);
        Some(next)
    }
}

#[derive(Default)]
struct MKAverage {
    top: MSet,
    bot: MSet,
    mid: MSet,
    que: VecDeque<i32>,
}
impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        Self {
            top: MSet::new(k as usize, true),
            bot: MSet::new(k as usize, false),
            mid: MSet::new((m - 2 * k) as usize, true),
            que: VecDeque::new(),
        }
    }
    fn put_chain(&mut self, num: i32) -> Option<i32> {
        let x = self.top.put(num)?;
        let y = self.bot.put(x)?;
        self.mid.put(y)
    }

    fn add_element(&mut self, num: i32) {
        if self.mid.size == self.mid.lim {
            let old = self.que.pop_front().unwrap();
            if self.top.remove_and_fill(old, &mut self.mid)
                || self.bot.remove_and_fill(old, &mut self.mid)
                || self.mid.remove(old)
            {}
        }
        self.put_chain(num);
        self.que.push_back(num)
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.mid.size < self.mid.lim {
            return -1;
        }
        (self.mid.sum / self.mid.lim as i64) as i32
    }
}

#[test]
fn test_1() {
    let mut avg = MKAverage::new(3, 1);
    avg.add_element(3);
    avg.add_element(1);
    assert_eq!(avg.calculate_mk_average(), -1);
    avg.add_element(10);
    assert_eq!(avg.calculate_mk_average(), 3);
    avg.add_element(5);
    avg.add_element(5);
    avg.add_element(5);
    assert_eq!(avg.calculate_mk_average(), 5);
}

#[test]
fn test_2() {
    let mut avg = MKAverage::new(7, 2);
    avg.add_element(1);
    avg.add_element(1);
    avg.add_element(3);
    avg.add_element(4);
    avg.add_element(5);
    avg.add_element(7);
    avg.add_element(7);
    assert_eq!(avg.calculate_mk_average(), 4);
    avg.add_element(100);
    assert_eq!(avg.calculate_mk_average(), 5);
    avg.add_element(100);
    assert_eq!(avg.calculate_mk_average(), 6);
    avg.add_element(100);
    assert_eq!(avg.calculate_mk_average(), 38);
}

