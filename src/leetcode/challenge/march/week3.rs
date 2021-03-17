use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

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

#[test]
fn lol() {
    println!("{:x}", 123u128)
}

pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    prices
        .iter()
        .fold([std::i32::MIN, 0], |[h, f], &x| [(f - x).max(h), (h + x - fee).max(f)])[1]
}
