use data::leetcode::ListNode;

pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let nums: std::collections::HashSet<_> = nums.into_iter().collect();
    let mut cur = &mut head;
    loop {
        match cur {
            Some(node) if nums.contains(&node.val) => *cur = node.next.take(),
            Some(node) => cur = &mut node.next,
            None => break head,
        }
    }
}
