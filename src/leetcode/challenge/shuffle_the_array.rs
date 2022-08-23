use std::iter::once;
pub fn shuffle(mut nums: Vec<i32>, n: i32) -> Vec<i32> {
    let pairs_from = |s| (s..).flat_map(|s|[s,s]);
    let firsts = once(1).chain(pairs_from(2));
    let seconds = pairs_from(n as usize);
    for (i, j) in firsts.zip(seconds).take((n as usize - 1) * 2){
        nums.swap(i, j)
    } 
    nums
}

#[test]
fn check1(){
    println!( "{:?}", shuffle(vec![1, 2, 3, 4, 11, 12, 13, 14], 4))
}
