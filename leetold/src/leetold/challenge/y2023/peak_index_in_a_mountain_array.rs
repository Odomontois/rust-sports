use std::fmt::Debug;

struct Solution;

const Q: f64 = 0.6180339887498;
fn between(from: usize, to: usize) -> usize {
    ((to as f64 - from as f64 + 1.0) * Q + from as f64) as usize
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct State<'a, A: Ord + Copy> {
    begin: usize,
    end: usize,
    mid: usize,
    is_right: bool,
    slice: &'a [A],
}

impl<'a, A: Ord + Copy> State<'a, A> {
    fn new(slice: &'a [A]) -> Self {
        Self {
            begin: 0,
            end: slice.len() - 1,
            mid: between(0, slice.len() - 1),
            is_right: true,
            slice,
        }
    }

    fn adjust(&mut self) {
        while self.slice[self.mid] <= self.slice[self.end] {
            let mid = between(self.mid, self.end);
            *self = State {
                begin: self.mid,
                mid,
                ..*self
            };
        }
        while self.slice[self.mid] <= self.slice[self.begin] {
            let mid = between(self.begin, self.mid);
            *self = State {
                mid,
                end: self.mid,
                ..*self
            };
        }
    }

    fn search_succeed(&self) -> bool {
        self.mid - self.begin <= 1 && self.end - self.mid <= 1
    }

    fn mids(&self) -> (usize, usize) {
        if self.is_right {
            (between(self.begin, self.mid), self.mid)
        } else {
            (self.mid, between(self.end, self.mid))
        }
    }

    fn go_right(&mut self, left_mid: usize, right_mid: usize) {
        *self = State {
            begin: left_mid,
            mid: right_mid,
            is_right: false,
            ..*self
        };
    }

    fn go_left(&mut self, left_mid: usize, right_mid: usize) {
        *self = State {
            mid: left_mid,
            end: right_mid,
            is_right: true,
            ..*self
        };
    }

    fn search(&mut self) -> usize {
        self.adjust();
        while !self.search_succeed() {
            let (left_mid, right_mid) = self.mids();
            if self.slice[left_mid] < self.slice[right_mid] {
                self.go_right(left_mid, right_mid);
            } else {
                self.go_left(left_mid, right_mid);
            }
        }
        self.mid
    }
}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        State::new(&arr).search() as i32
    }
}

#[test]
fn example1() {
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 1, 0]));
}

#[test]
fn example2() {
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]));
}

#[test]
fn example2_5() {
    assert_eq!(2, Solution::peak_index_in_mountain_array(vec![0, 2, 3, 1, 0]));
}

#[test]
fn example3() {
    assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]));
}

#[test]
fn wa1() {
    assert_eq!(
        12,
        Solution::peak_index_in_mountain_array(vec![
            8, 18, 24, 31, 37, 42, 43, 56, 65, 73, 93, 98, 100, 98, 76, 72, 69, 24, 23
        ])
    );
}
