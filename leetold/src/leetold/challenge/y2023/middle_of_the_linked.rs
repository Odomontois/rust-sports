use data::leetcode::ListNode;
use std::iter::successors;

pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let n = successors(head.as_ref(), |opt| opt.next.as_ref()).count();
    for _ in 0..n / 2 {
        head = head.and_then(|b| b.next)
    }
    head
}
