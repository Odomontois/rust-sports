pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
    let left_to_right = fruits.iter().map(|v| [v[0], v[1]]);
    let right_to_left = fruits.iter().rev().map(|v| [-v[0], v[1]]);
    back_and_forth(left_to_right, start_pos, k).max(back_and_forth(right_to_left, -start_pos, k))
}

pub fn back_and_forth(fruits: impl Iterator<Item = [i32; 2]> + Clone, pos: i32, k: i32) -> i32 {
    let mut sum = 0;
    let mut prev = pos;
    let mut best = forth(fruits.clone(), pos, k);
    for [p, c] in fruits.clone(){
        
    }
    best
}

pub fn forth(fruits: impl Iterator<Item = [i32; 2]>, pos: i32, k: i32) -> i32 {
    fruits.skip_while(|x| x[0] < pos).take_while(|x| x[0] <= pos + k).map(|x| x[1]).sum()
}
