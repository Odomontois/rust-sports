use data::leetcode::ListNode;

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = None;
    while let Some(b) = head {
        res = Some(Box::new(ListNode { val: b.val, next: res }));
        head = b.next
    }
    res
}
