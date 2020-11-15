#![feature(array_map)]

use crate::leetcode::challenge::november::week2::{init_comb, pig_it, poor_pigs};

mod lol;
mod leetcode;


fn main() {
    for i in 1..5 {
        println!("{} {:?}", i, pig_it(i, &init_comb()).into_iter().enumerate().collect::<Vec<_>>())
    }
    println!("{}", poor_pigs(4, 15, 15));
}
