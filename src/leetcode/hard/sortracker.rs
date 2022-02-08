use std::{cmp::Reverse, collections::BinaryHeap};

type City = (Reverse<i32>, String);
#[derive(Default)]
struct SORTracker {
    top: BinaryHeap<City>,
    bot: BinaryHeap<Reverse<City>>,
}

impl SORTracker {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, name: String, score: i32) {
        let city = (Reverse(score), name);
        if Some(&city) < self.top.peek() {
            for t in self.top.pop() {
                self.bot.push(Reverse(t))
            }
            self.top.push(city)
        } else {
            self.bot.push(Reverse(city))
        }
    }

    fn get(&mut self) -> String {
        for Reverse(city) in self.bot.pop() {
            self.top.push(city.clone());
            return city.1;
        }
        String::default()
    }
}
