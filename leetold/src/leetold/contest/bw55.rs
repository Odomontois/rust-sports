pub fn can_be_increasing(nums: Vec<i32>) -> bool {
    (0..nums.len()).any(|i| {
        let mut x = nums.clone();
        x.remove(i);
        x.windows(2).all(|v| v[0] < v[1])
    })
}

pub fn remove_occurrences(mut s: String, part: String) -> String {
    while let Some(i) = s.find(&part) {
        s.replace_range(i..i + part.len(), "")
    }
    s
}

// pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
//     let mut max = 0;
//     let mut min = 0;
//     for x in nums {
//         max = max.max(x as i64 - min);
//         min = min.min(x as i64 - max);
//     }
//     max
// }

pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    nums.into_iter()
        .map(<_>::into)
        .fold((0i64, 0), |(a, b), x: i64| (a.max(x - b), b.min(x - a)))
        .0
}

#[test]
fn max_alt() {
    assert_eq!(max_alternating_sum(vec![5, 6, 7, 8]), 8);
    assert_eq!(max_alternating_sum(vec![6, 2, 1, 2, 4, 5]), 10);
}
use std::{
    collections::{BTreeSet, HashMap},
    convert::TryInto,
};

#[derive(Default, Clone)]
struct MovieRentingSystem {
    prices: HashMap<(i32, i32), i32>,
    catalog: HashMap<i32, BTreeSet<(i32, i32)>>,
    rented: BTreeSet<(i32, i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(_: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut me = Self::default();
        for entry in entries {
            if let Ok::<&[i32; 3], _>(&[shop, movie, price]) = entry.as_slice().try_into() {
                me.prices.insert((shop, movie), price);
                me.catalog.entry(movie).or_default().insert((price, shop));
            }
        }
        me
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        let opt = self.catalog.get(&movie).into_iter().flatten();
        opt.take(5).map(|&(_price, shop)| shop).collect()
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = self.prices[&(shop, movie)];
        self.catalog.entry(movie).or_default().remove(&(price, shop));
        self.rented.insert((price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = self.prices[&(shop, movie)];
        self.rented.remove(&(price, shop, movie));
        self.catalog.entry(movie).or_default().insert((price, shop));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        let it = self.rented.iter();
        it.take(5).map(|&(_price, shop, movie)| vec![shop, movie]).collect()
    }
}
