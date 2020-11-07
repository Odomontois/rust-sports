use std::rc::Rc;

use crate::leetcode::random::largest_divisible_subset;

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


fn kek(x: &mut i32) {
    *x += 1
}

fn main() {
    println!("{:?}", largest_divisible_subset(vec![4, 8, 10, 240, 9, 27, 81, 810]))
}
