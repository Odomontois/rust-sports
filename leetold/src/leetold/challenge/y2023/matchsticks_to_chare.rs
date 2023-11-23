// https://leetcode.com/problems/matchsticks-to-square/

pub fn makesquare(matchsticks: Vec<i32>) -> bool {
    let sum: i32 = matchsticks.iter().sum();
    if sum % 4 != 0 || matchsticks.iter().any(|&x| x > sum / 4) {
        return false;
    }
    check(&matchsticks, (1 << matchsticks.len()) - 1, sum / 4).is_some()
}

fn check(xs: &[i32], mask: u32, amt: i32) -> Option<()> {
    if mask == 0 {
        return Some(());
    }
    let mut elems = xs.iter().copied().enumerate().filter(|(i, _)| mask & (1 << i) != 0);
    let (i, first) = elems.next()?;
    check_next(xs, mask ^ (1 << i), amt, amt - first, elems)
}

fn check_next<I>(xs: &[i32], mask: u32, amt: i32, rem: i32, mut elems: I) -> Option<()>
where
    I: Iterator<Item = (usize, i32)> + Clone,
{
    if rem == 0 {
        return check(xs, mask, amt);
    }
    while let Some((i, x)) = elems.next() {
        if x <= rem && check_next(xs, mask ^ (1 << i), amt, rem - x, elems.clone()).is_some() {
            return Some(());
        }
    }
    None
}

#[test]
fn wa1() {
    assert!(makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3]))
}
