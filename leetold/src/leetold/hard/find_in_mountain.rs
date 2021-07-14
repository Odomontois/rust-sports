// use std::ops::Index;
// use crate::data::leetcode::Tree;
//
// pub struct MountainArray(Vec<i32>);
//
// impl MountainArray {
//     pub fn get(&self, index: i32) -> i32 { self.0[index as usize] }
//     pub fn length(&self) -> i32 { self.0.len() as i32 }
// }
//
//
// pub fn find_in_mountain_array(target: i32, arr: &MountainArray) -> i32 {
//     unimplemented!()
// }
//
//
// // fn search_max(arr: &MountainArray) -> i32 {
// //     let gold: f64 = (5f64.sqrt() + 1f64) / 2f64;
// //     let gmedi = move |l: i32, r: i32| l + ((r - l) as f64 * gold) as i32;
// //     let (mut start, mut end) = (0, arr.length() - 1);
// //     let mut mid = gmedi(start, end);
// //     let (mut sv, ev, mv) = (arr.get(start), arr.get(end), arr.get(mid));
// //     while end - start >= 2 {
// //         if mid - start > end - mid {
// //             let m2 = gmedi(start, mid);
// //             let m2v = arr.get(m2);
// //             if m2v > mv {}
// //         } else {
// //             let m2 = gmedi(mid, end);
// //             let m2v = arr.get(m2);
// //         }
// //     }
// //
// //     mid
// // }
//
//
//
