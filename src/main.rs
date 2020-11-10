use crate::leetcode::haystack::{KMP, str_str};

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
    println!("{:?}", KMP::build( "bba".to_string()));
    println!("{:?}", str_str("abba".to_string(), "bba".to_string()))
}
