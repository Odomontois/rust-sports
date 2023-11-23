pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = Vec::with_capacity(nums.iter().map(|v| v.len()).sum());
    let mut nums = nums
        .into_iter()
        .map(|v| std::cell::RefCell::new(v.into_iter().peekable()))
        .peekable();
    let mut its = vec![];
    while nums.peek().is_some() || !its.is_empty() {
        its.extend(nums.next());
        its.retain(|it| it.borrow_mut().peek().is_some());
        for v in its.iter_mut().rev() {
            if let Some(x) = v.borrow_mut().next() {
                res.push(x)
            }
        }
    }
    res
}
