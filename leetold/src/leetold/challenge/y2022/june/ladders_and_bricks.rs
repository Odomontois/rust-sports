use std::{cmp::Reverse, collections::BinaryHeap};

pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
    let mut ups = BinaryHeap::new();
    let ws = heights.windows(2).enumerate().filter_map(|h| match h {
        (i, &[p, h]) if h > p => Some((i as i32, p, h)),
        _ => None,
    });
    for (i, prev, h) in ws {
        ups.push(Reverse(h - prev));
        if ups.len() > ladders as usize {
            if let Some(Reverse(g)) = ups.pop() {
                bricks -= g;
                if bricks < 0 {
                    return i;
                }
            }
        }
    }
    heights.len() as i32 - 1
}
