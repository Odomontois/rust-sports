use std::{
    io::{self, prelude::*},
    usize,
};

pub fn main() {
    median_main()
}

pub fn median_main() {
    let sin = io::stdin();
    let _ = MedianSort::new(sin.lock().lines().flatten());
}

#[allow(dead_code)]
struct MedianSort<A> {
    inp: A,
    t: usize,
    n: usize,
    q: usize,
}

#[allow(dead_code)]
impl<A> MedianSort<A>
where
    A: Iterator<Item = String>,
{
    fn new(mut inp: A) -> Option<Self> {
        let l = inp.next()?;
        let mut xs: _ = l.split(" ").flat_map(str::parse);
        Some(Self {
            inp,
            t: xs.next()?,
            n: xs.next()?,
            q: xs.next()?,
        })
    }

    fn request(&mut self, x: usize, y: usize, z: usize) -> usize {
        println!("{} {} {}", x, y, z);
        self.inp.next().and_then(|s| s.parse().ok()).unwrap()
    }

    fn sort(&mut self, _: Vec<u32>) -> Vec<u32> {
        //       | a b x | a x y |
        // -----------------------------------------
        // abxy  |   b   |   x   |
        // axby  |   x   |   x   |
        // xaby  |   a   |   a   |
        // axyb  |   x   |   x   |
        // xayb  |   a   |   a   |
        // xyab  |   a   |   y   |
        // ----------------------------
        todo!()
    }

    fn work(&mut self) {
        for _ in 0..self.t {
            let s = self.inp.next().unwrap();
            let nums = s.split(" ").flat_map(|x| x.parse()).collect();
            let sorted = self
                .sort(nums)
                .into_iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", sorted);
        }
    }
}
