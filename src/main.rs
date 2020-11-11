use crate::leetcode::challenge::november::week2::valid_square;

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
    println!("{}", valid_square(vec![1, 1], vec![0, 0], vec![1, 0], vec![0, 1]))
}
