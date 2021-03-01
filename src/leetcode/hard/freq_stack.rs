use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

struct FrequencyStack<A> {
    queue: BinaryHeap<(usize, usize, A)>,
    freqs: HashMap<A, usize>,
    idx: usize,
}

impl<A: Ord + Hash + Clone> FrequencyStack<A> {
    fn new() -> Self {
        FrequencyStack {
            queue: BinaryHeap::new(),
            freqs: HashMap::new(),
            idx: 0,
        }
    }

    fn push(&mut self, x: A) {
        let c = self.freqs.entry(x.clone()).or_insert(0);
        *c += 1;
        self.queue.push((*c, self.idx, x));
        self.idx += 1;
    }

    fn pop(&mut self) -> A {
        let (mut freq, _, x) = self.queue.pop().unwrap();
        freq -= 1;
        if freq != 0 {
            *self.freqs.get_mut(&x).unwrap() -= 1;
        } else {
            self.freqs.remove(&x);
        }
        x
    }
}

type FreqStack = FrequencyStack<i32>;
