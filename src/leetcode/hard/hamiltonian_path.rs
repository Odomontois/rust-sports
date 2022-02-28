use std::{collections::VecDeque, fmt::Display};

pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    Ham::new(graph).calc().map(|x| x as i32).unwrap_or(-1)
}

struct Ham {
    graph: Vec<Vec<i32>>,
    cache: Vec<Vec<Option<u8>>>,
    queue: VecDeque<Item>,
}

#[derive(Clone, Copy, Debug)]
struct Item {
    index: usize,
    seen: usize,
    steps: u8,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Item { seen, steps, index } = self;

        let seen_vec: Vec<_> = (0..12).filter(|&i| seen & (1 << i) != 0).collect();
        write!(f, "{index} {seen_vec:?} = {steps}")
    }
}

impl Item {
    fn prevs<'a>(&'a self, ixs: impl Iterator<Item = &'a i32> + 'a) -> impl Iterator<Item = Item> + 'a {
        ixs.map(|&x| x as usize)
            .filter(move |&i| self.seen & (1 << i) != 0 && i != self.index)
            .flat_map(move |index| {
                let steps = self.steps + 1;
                let item = |seen| Item { seen, steps, index };
                [item(self.seen), item(self.seen ^ (1 << self.index))]
            })
    }

    fn is_first(&self) -> bool {
        self.seen == 1 << self.index
    }
}

impl Ham {
    fn new(graph: Vec<Vec<i32>>) -> Self {
        let cache = vec![vec![None; 1 << graph.len()]; graph.len()];
        let queue = VecDeque::new();
        Self { graph, cache, queue }
    }

    fn init(&mut self) {
        let seen = (1 << self.graph.len()) - 1;
        for index in 0..self.graph.len() {
            self.cache[index][seen] = Some(0);
            self.queue.push_back(Item { index, seen, steps: 0 })
        }
    }

    fn push_prevs(&mut self, item: Item) {
        for prev in item.prevs(self.graph[item.index].iter()) {
            let c = &mut self.cache[prev.index][prev.seen];
            if c.is_none() {
                *c = Some(item.steps);
                self.queue.push_back(prev);
            }
        }
    }

    fn calc(&mut self) -> Option<u8> {
        self.init();
        while let Some(item) = self.queue.pop_front() {
            if item.is_first() {
                return Some(item.steps);
            }
            self.push_prevs(item)
        }
        None
    }
}

#[test]
fn test1() {
    assert_eq!(4, shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]]))
}

#[test]
fn test2() {
    assert_eq!(
        4,
        shortest_path_length(vec![vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]])
    )
}

#[test]
fn test3() {
    assert_eq!(
        6,
        shortest_path_length(vec![vec![1], vec![0, 2, 4], vec![1, 3], vec![2], vec![1, 5], vec![4]])
    )
}

#[test]
fn test4() {
    assert_eq!(
        11,
        shortest_path_length(vec![
            vec![6, 9],
            vec![6, 8],
            vec![6, 7],
            vec![6, 10],
            vec![8],
            vec![10],
            vec![0, 1, 2, 3, 8],
            vec![2, 8, 9],
            vec![1, 4, 6, 7],
            vec![0, 7, 10],
            vec![3, 5, 9]
        ])
    )
}
