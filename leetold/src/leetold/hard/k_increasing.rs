// https://leetcode.com/problems/minimum-operations-to-make-the-array-k-increasing

pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
    let (n, k) = (arr.len(), k as usize);
    let it = |i| minops((0..).map(|j| j * k + i).take_while(|&j| j < n).map(|j| arr[j]));
    (0..k).map(it).sum()
}

fn minops(xs: impl Iterator<Item = i32>) -> i32 {
    let mut count = 0;
    let mut stack: Vec<i32> = vec![];
    for x in xs {
        count += 1;
        if let Err(i) = stack.binary_search_by(|&y| (y * 2).cmp(&(x * 2 + 1))) {
            if i == stack.len() {
                stack.push(x)
            } else {
                stack[i] = x;
            }
        }
    }
    count - stack.len() as i32
}
