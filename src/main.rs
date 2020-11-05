use std::rc::Rc;

mod lol;
mod leetcode;

use leetcode::bw38::*;
use crate::leetcode::week::november1::{max_power, find_min_height_trees, find_min_height_trees_impl};

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


fn kek(x: &mut i32) {
    *x += 1
}

fn main() {
    println!("{:?}", find_min_height_trees_impl( vec![[0,1],[0,2],[2,3],[2,4],[2,5],[4,6],[0,7],[4,8],[5,9],[7,10],[6,11],[0,12],[0,13],[3,14]], true));
}
