use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    marker::PhantomData,
    rc::Rc,
};

type City = (Reverse<i32>, String);

struct Pool<'a, A> {
    elems: Vec<A>,
    cmp_cache: HashMap<(usize, usize), Ordering>,
    ph: PhantomData<&'a u8>,
}

struct PoolElem<'a, A> {
    pool: Rc<RefCell<Pool<'a, A>>>,
    ix: usize,
}
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
            if let Some(t) = self.top.pop() {
                self.bot.push(Reverse(t))
            }
            self.top.push(city)
        } else {
            self.bot.push(Reverse(city))
        }
    }

    fn get(&mut self) -> String {
        if let Some(Reverse(city)) = self.bot.pop() {
            self.top.push(city.clone());
            return city.1;
        }
        String::default()
    }
}
