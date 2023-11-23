// object Solution {
//     case class State(elems: Vector[Int] = Vector.empty, last: Int = Int.MinValue){
//         def end: Boolean = elems.forall(_ >= 3)
//         def feed(elem: Int, count: Int): Option[State] =
//             if(elem > last + 1) Option.when(end)(State(Vector.fill(count)(1), elem))
//             else if (count >= elems.size)
//                 Some(State(Vector.fill(count - elems.size)(1) ++ elems.map(_ + 1), elem))
//             else {
//                 val (remains, drop) = elems.splitAt(count)
//                 Option.when(drop.forall(_ >= 3))(State(remains.map(_ + 1), elem))
//             }
//     }

//     def isPossible(nums: Array[Int]): Boolean = {
//         def go(i: Int, state: State, cur: Int, cnt: Int): Boolean =
//             if(i == nums.length) state.feed(cur, cnt).exists(_.end) else {
//                 val x = nums(i)
//                 if (x == cur) go(i + 1, state, cur, cnt + 1)
//                 else state.feed(cur, cnt) match {
//                     case None => false
//                     case Some(st) => go(i + 1, st, x, 1)
//                 }
//            }
//         go(0, State(), Int.MinValue, 0)
//     }
// }

use std::collections::VecDeque;

pub fn is_possible(nums: Vec<i32>) -> bool {
    let mut cur: i32 = i32::min_value();
    let mut cnt: usize = 0;
    let mut state = State::new();

    for num in nums {
        if num == cur {
            cnt += 1;
            continue;
        }
        if !state.feed(cur, cnt){
            return false
        }
        cur = num;
        cnt = 1;
    }
    state.feed(cur, cnt) && state.end()
}

#[derive(Default)]
struct State {
    elems: VecDeque<i32>,
    last: i32,
}

impl State {
    fn new() -> Self{
        Self { elems: VecDeque::new(), last: i32::min_value() }
    }

    fn end(&self) -> bool {
        self.elems.iter().all(|x| *x >= 3)
    }

    fn feed(&mut self, elem: i32, count: usize) -> bool {
        if elem > self.last + 1 {
            if !self.end() {
                return false;
            }
            self.elems = vec![1; count].into();
            self.last = elem;
            return true;
        }

        for _ in self.elems.len()..count {
            self.elems.push_front(0);
        }

        for _ in count..self.elems.len() {
            if let Some(x) = self.elems.pop_back() {
                if x < 3 {
                    return false;
                }
            }
        }

        for x in &mut self.elems {
            *x += 1;
        }
        self.last = elem;

        true
    }
}
