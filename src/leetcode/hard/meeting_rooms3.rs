// https://leetcode.com/problems/meeting-rooms-iii/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    iter::FromIterator,
};


pub fn most_booked<V: AsRef<[i32]>>(n: i32, meetings: Vec<V>) -> i32 {
    let mut solution = MostBooked::new(n, meetings, |v| (v.as_ref()[0] as u64, v.as_ref()[1] as u64));
    solution.emulate();
    solution.result()
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Event {
    Free { room: usize },
    Meeting { duration: u64 },
}


#[derive(Debug)]
struct MostBooked {
    waiting: VecDeque<u64>,
    events: BinaryHeap<Reverse<(u64, Event)>>,
    free_rooms: BinaryHeap<Reverse<usize>>,
    counter: Vec<u32>,
}

impl MostBooked {
    fn new<V>(n: i32, meetings: Vec<V>, f: impl Fn(V) -> (u64, u64)) -> Self {
        let make_meeting = |v: V| {
            let (start, end) = f(v);
            Reverse((start, Event::Meeting { duration: end - start }))
        };
        let meet_it = meetings.into_iter().map(make_meeting);
        Self {
            waiting: VecDeque::new(),
            events: BinaryHeap::from_iter(meet_it),
            free_rooms: (0..n as usize).map(Reverse).collect(),
            counter: vec![0; n as usize],
        }
    }

    fn start_meeting(&mut self, free: u64, room: usize) {
        self.counter[room] += 1;
        self.events.push(Reverse((free, Event::Free { room })))
    }

    fn handle_event(&mut self, t: u64, event: Event) {
        match event {
            Event::Meeting { duration } => {
                if let Some(Reverse(room)) = self.free_rooms.pop() {
                    self.start_meeting(t + duration, room)
                } else {
                    self.waiting.push_back(duration)
                }
            }
            Event::Free { room } => {
                if let Some(duration) = self.waiting.pop_front() {
                    self.start_meeting(t + duration, room)
                } else {
                    self.free_rooms.push(Reverse(room))
                }
            }
        }
    }

    fn emulate(&mut self) {
        while let Some(Reverse((t, event))) = self.events.pop() {
            self.handle_event(t, event)
        }
    }

    fn result(&self) -> i32 {
        self.counter
            .iter()
            .enumerate()
            .max_by_key(|&(num, count)| (count, Reverse(num)))
            .map(|(idx, _)| idx as i32)
            .unwrap_or(-1)
    }
}

#[test]
fn example1() {
    assert_eq!(0, most_booked(2, vec![[0, 10], [1, 5], [2, 7], [3, 4]]))
}

#[test]
fn example2() {
    assert_eq!(1, most_booked(3, vec![[1, 20], [2, 10], [3, 5], [4, 9], [6, 8]]))
}

#[test]
fn wa1() {
    assert_eq!(
        0,
        most_booked(
            4,
            vec![
                [48, 49],
                [22, 30],
                [13, 31],
                [31, 46],
                [37, 46],
                [32, 36],
                [25, 36],
                [49, 50],
                [24, 34],
                [6, 41]
            ]
        )
    )
}
