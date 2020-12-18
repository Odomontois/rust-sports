pub trait RadixSort: Sized {
    type Item: RadixU8;
    fn rearrange<F>(&mut self, f: F) where F: Fn(&Self::Item) -> u8;

    fn radix_sort(&mut self) { Self::Item::do_radix_sort(self); }
}

pub trait RadixU8: Sized {
    fn do_radix_sort<A: RadixSort<Item=Self>>(a: &mut A);
}

impl RadixU8 for i32 {
    fn do_radix_sort<A: RadixSort<Item=Self>>(a: &mut A) {
        a.rearrange(|&x| x as u8);
        a.rearrange(|&x| (x >> 8) as u8);
        a.rearrange(|&x| (x >> 16) as u8);
        a.rearrange(|&x| ((x >> 24) ^ (1 << 7)) as u8); //flip the sign bit
    }
}

impl<A: RadixU8> RadixSort for Vec<A> {
    type Item = A;

    fn rearrange<F>(&mut self, f: F) where F: Fn(&A) -> u8 {
        let mut ix = [0usize; 256];
        for e in self.iter() {
            ix[f(e) as usize] += 1
        }
        let mut elems = Vec::with_capacity(256);
        for i in 0..256 { elems.push(Vec::with_capacity(ix[i])) }
        for e in self.drain(..) {
            let i = f(&e) as usize;
            elems[i].push(e);
            ix[i] += 1;
        }
        for v in elems { self.extend(v) }
    }
}

#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};
    use super::*;

    #[test]
    fn rearrange_test() {
        let mut x = vec![1, 2, 3, 4, 5, 6, 7, 8];
        x.rearrange(|&e| (e % 2) as u8);
        assert_eq!(x, vec![2, 4, 6, 8, 1, 3, 5, 7]);
        x.rearrange(|&e| (e % 3) as u8);
        assert_eq!(x, vec![6, 3, 4, 1, 7, 2, 8, 5]);
    }

    #[test]
    fn sort_test() {
        let mut rng = thread_rng();
        let mut x: Vec<i32> = vec![];
        for _ in 0..10000 { x.push(rng.gen()) }
        let mut y = x.clone();
        x.radix_sort();
        y.sort();
        assert_eq!(x, y);
    }
}
