use std::{cmp::Reverse, collections::BinaryHeap, iter::from_fn};

fn dbl_linear(n: u32) -> u32{
    let mut q = BinaryHeap::new();
    let mut prev = 0;
    q.push( Reverse(1));
    let mut is = from_fn(|| {
        let mut cur = prev;
        while cur == prev{
            cur = q.pop()?.0;
        }
        prev = cur;
        q.push(Reverse(2 * cur + 1));
        q.push(Reverse(3 * cur + 1));
        Some(cur)
    });

    is.nth(n as usize).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::dbl_linear;
    fn testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }
    
    #[test]
    fn basics_dbl_linear() {
        testing(10, 22);
        testing(20, 57);
        testing(30, 91);
        testing(50, 175);
        testing(100, 447);
    }
}