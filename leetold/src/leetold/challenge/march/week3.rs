use std::iter::successors;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

#[cfg(test)]
use rand::{thread_rng, RngCore};

struct Codec {
    elems: RefCell<HashMap<u64, String>>,
    next: Cell<u64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {
            elems: RefCell::new(HashMap::new()),
            next: Cell::new(0),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&self, long_url: String) -> String {
        self.next.set(self.next.get() + 1);
        self.elems.borrow_mut().insert(self.next.get(), long_url);
        format!("http://ti.ny/{:x}", self.next.get())
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        let s = short_url.trim_start_matches("http://ti.ny/");
        let elem = u64::from_str_radix(s, 16).unwrap();
        self.elems.borrow()[&elem].clone()
    }
}

pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    prices
        .iter()
        .fold([std::i32::MIN, 0], |[h, f], &x| [(f - x).max(h), (h + x - fee).max(f)])[1]
}

pub fn wiggle_max_length2(nums: Vec<i32>) -> i32 {
    let [(asc, _), (desc, _)] = nums
        .into_iter()
        .fold([(0, 10000), (0, -1)], |[(asc, pa), (desc, pd)], num| {
            [
                if num > pd { (desc + 1, num) } else { (asc, pa) },
                if num < pa { (asc + 1, num) } else { (desc, pd) },
            ]
        });
    asc.max(desc)
}

pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    let [(asc, _), (desc, _)] = nums
        .into_iter()
        .fold([(0, 10000), (0, -1)], |[(asc, pa), (desc, pd)], num| {
            [
                if num > pd { (desc + 1, num) } else { (asc, pa) },
                if num < pa { (asc + 1, num) } else { (desc, pd) },
            ]
        });
    asc.max(desc)
}

#[test]
fn test() {
    let s = "query questionTags($skipCompanyTags: Boolean = false) {\n  questionTopicTags {\n    edges {\n      node {\n        name\n        translatedName\n        slug\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n  questionCompanyTags @skip(if: $skipCompanyTags) {\n    edges {\n      node {\n        name\n        translatedName\n        slug\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n}\n";
    println!("{}", s)
}

struct UndergroundSystem {
    current: HashMap<i32, (String, i32)>,
    stat: HashMap<(String, String), (i64, u32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self {
            current: HashMap::new(),
            stat: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, start: String, t: i32) {
        self.current.insert(id, (start, t));
    }

    fn check_out(&mut self, id: i32, end: String, t2: i32) {
        let (start, t1) = self.current.remove(&id).unwrap();
        let (sum, count) = self.stat.entry((start, end)).or_insert((0, 0));
        *sum += (t2 - t1) as i64;
        *count += 1;
    }

    fn get_average_time(&self, start: String, end: String) -> f64 {
        let (sum, count) = self.stat[&(start, end)];
        sum as f64 / count as f64
    }
}

lazy_static! {
    static ref POWS: Vec<u64> = (0..30).map(|x| 1 << x).map(digit_map).collect();
}
pub fn reordered_power_of2(n: i32) -> bool {
    POWS.contains(&digit_map(n))
}

fn digit_map(n: i32) -> u64 {
    successors(Some(n), |&x| Some(x / 10))
        .take_while(|&x| x > 0)
        .map(|x| x % 10)
        .fold(0, |m, c| {
            let mask = 0b1111 << 4 * c;
            let count = ((m & mask) >> 4 * c) + 1;
            m & (!0 ^ mask) | (count << 4 * c)
        })
}

#[test]
fn check_random() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let x = (rng.next_u32() % 1000_000_000) as i32;
        let mut dc = vec![0; 10];
        for c in x.to_string().chars() {
            let cc = c as usize - '0' as usize;
            dc[cc] += 1;
        }
        let mut m = 0;
        for (i, x) in dc.iter().enumerate() {
            m |= x << 4 * i
        }
        assert_eq!(m, digit_map(x))
    }
}
