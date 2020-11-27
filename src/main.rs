#![feature(array_map)]

use crate::leetcode::challenge::november::week4::SolLongest;

mod lol;
mod leetcode;
mod data;


fn main() {
    println!("{:?}", SolLongest::longest_substring("aaabb".to_string(), 3));
}
