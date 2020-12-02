use crate::leetcode::data::List;
use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;

struct RandomChoose<R: Rng> { items: Vec<i32>, rng: R }

type Solution = RandomChoose<ThreadRng>;

impl Solution {
    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(mut head: List) -> Self {
        let mut items = Vec::new();
        while let Some(h) = head {
            items.push(h.val);
            head = h.next;
        }
        RandomChoose { items, rng: thread_rng() }
    }
}

impl<R: Rng> RandomChoose<R> {
    /** Returns a random node's value. */
    fn get_random(&mut self) -> i32 {
        let i = self.rng.gen_range(0, self.items.len());
        self.items[i]
    }
}