use std::rc::Rc;

mod lol;
mod leetcode;

use leetcode::bw38::*;
use crate::leetcode::week::november1::max_power;

// fn main() {
// //    let nums = vec![0, 0];
//     let nums = vec![-1, 0, 1, 2, -1, -4];
//     let res = leetcode::sum_3::Solution::three_sum(nums);
//     println!("{:?}", res);
// }

// pub enum CherryTree<T> {
//     Empty,
//     Leaf(Rc<T>),
//     Branch(Option<RC<T>>, Box<CherryTree<RC<(T, T)>>>, Option<RC<T>>),
// }

fn main() {
    println!("{}", max_power("leetcode".to_string()));
}
