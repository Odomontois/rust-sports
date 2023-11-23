use std::{collections::BinaryHeap, cmp::Reverse};

struct SeatManager(BinaryHeap<Reverse<i32>>);

impl SeatManager {

    fn new(n: i32) -> Self {
        Self((1..=n).map(Reverse).collect::<Vec<_>>().into())
    }
    
    fn reserve(&mut self) -> i32 {
        self.0.pop().map_or(0, |Reverse(x)| x)
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.0.push(Reverse(seat_number))
    }
}