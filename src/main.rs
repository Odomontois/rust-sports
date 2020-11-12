#![feature(array_map)]

use crate::leetcode::challenge::november::week2::permute_unique;

mod lol;
mod leetcode;

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
    println!("{:#?}", permute_unique(vec![1, 2, 3, 4, 5]))
}
