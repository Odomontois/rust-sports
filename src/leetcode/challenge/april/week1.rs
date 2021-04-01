use crate::leetcode::data::List;

#[cfg(test)]
use crate::leetcode::data::list;

pub fn is_palindrome(mut head: List) -> bool {
    let l = length(&head);
    let mut left = None;
    for _ in 0..l / 2 {
        let mut b = head.unwrap();
        head = b.next;
        b.next = left;
        left = Some(b)
    }
    if l % 2 == 1 {
        head = head.unwrap().next;
    }
    while let (Some(lb), Some(rb)) = (left, head) {
        if lb.val != rb.val {
            return false;
        }
        left = lb.next;
        head = rb.next;
    }

    true
}

fn length(mut lst: &List) -> usize {
    let mut res = 0;
    while let Some(b) = lst {
        res += 1;
        lst = &b.next;
    }
    res
}
#[test]
fn test() {
    assert!(is_palindrome(list(&[1, 2, 2, 1])));
    assert!(is_palindrome(list(&[1, 2, 3, 2, 1])));
    assert!(!is_palindrome(list(&[1, 2, 3, 3, 1])));
    assert!(!is_palindrome(list(&[1, 2,])));
}
