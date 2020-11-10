#[allow(dead_code)]
pub fn max_power(s: String) -> i32 {
    s.chars().scan((1i32, '\0'), |(count, prev), c| {
        if *prev == c { *count += 1 } else { *count = 1 }
        *prev = c;
        Some(*count)
    }).max().unwrap_or(0)
}

#[allow(dead_code)]
pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
    let mut counts = [0, 0];
    position.into_iter().for_each(|x| counts[(x % 2) as usize] += 1);
    counts[0].min(counts[1])
}


struct Search { nums: Vec<i32>, threshold: i32 }

impl Search {
    fn go(&self, from: i32, to: i32) -> i32 {
        if (to - from) <= 1 { to } else {
            let m = (from + to) / 2;
            let res: i32 = self.nums.iter().map(|&n| (n + m - 1) / m).sum();
            if res <= self.threshold { self.go(from, m) } else { self.go(m, to) }
        }
    }
}
#[allow(dead_code)]
pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    Search { nums, threshold }.go(0, 2000_000)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

type IList = Option<Box<ListNode>>;
use std::ops::Add;
struct ListNum(IList);

impl ListNum {
    pub fn new(list: IList) -> ListNum { ListNum(Self::reverse(list)) }
    fn unpack(list: IList) -> Option<(i32, IList)> {
        let bx = list?;
        let ListNode { val, next } = *bx;
        Some((val, next))
    }
    fn reverse(mut list: IList) -> IList {
        let mut res = None;
        while let Some((val, next)) = Self::unpack(list) {
            list = next;
            res = Some(Box::new(ListNode { val, next: res }))
        }
        res
    }
    fn into_list(self) -> IList { Self::reverse(self.0) }
}

impl Add for ListNum {
    type Output = ListNum;

    fn add(self, rhs: Self) -> Self::Output {
        let mut rem = 0;
        let ListNum(mut x) = self;
        let ListNum(mut y) = rhs;
        let mut res = None;
        while x.is_some() || y.is_some() || rem > 0 {
            let (xv, x1) = Self::unpack(x).unwrap_or((0, None));
            x = x1;
            let (yv, y1) = Self::unpack(y).unwrap_or((0, None));
            y = y1;
            let v = xv + yv + rem;
            rem = v / 10;
            res = Some(Box::new(ListNode {
                val: v % 10,
                next: res,
            }))
        }
        ListNum::new(res)
    }
}

#[allow(dead_code)]
pub fn add_two_numbers(l1: IList, l2: IList) -> IList {
    (ListNum::new(l1) + ListNum::new(l2)).into_list()
}

