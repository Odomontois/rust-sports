pub trait RadixSort {
    type Item;

    fn rearrange<F>(&mut self, f: F) where F: Fn(&Self::Item) -> u8;

    fn radix_sort(&mut self) where Self::Item: RadixU8 {
        self.radix_sort_with(|x| x);
    }
    fn radix_sort_with<F, A: RadixU8>(&mut self, f: F) where F: Fn(&Self::Item) -> &A {
        A::do_radix_sort_with(self, f);
    }
}

pub trait RadixU8 {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self;
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

impl<A: RadixU8> RadixSort for [A] {
    type Item = A;

    fn rearrange<F>(&mut self, f: F) where F: Fn(&A) -> u8 {
        let mut ix = [0usize; 256];
        for e in self.iter() {
            ix[f(e) as usize] += 1
        }
        let mut n = 0;
        for i in ix.iter_mut() {
            let x = *i;
            *i = n;
            n += x;
        }
        let mut elems = vec![0usize; n];
        let mut done = vec![false; n];
        for (i, e) in self.iter().enumerate() {
            let b = f(e) as usize;
            elems[ix[b]] = i;
            ix[b] += 1;
        }
        for i in 0..n {
            if done[i] { continue; }
            let start = i;
            let mut cur = start;
            loop {
                done[cur] = true;
                let next = elems[cur];
                if next == start { break; };
                self.swap(cur, next);
                cur = next;
            }
        }
    }
}


#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};
    use super::*;
    use std::mem::size_of;

    #[test]
    fn size_test() {
        println!("{} {}", size_of::<char>(), size_of::<usize>())
    }

    #[test]
    fn rearrange_vec_test() {
        let mut x = vec![1, 2, 3, 4, 5, 6, 7, 8];
        x.rearrange(|&e| (e % 2) as u8);
        assert_eq!(x, vec![2, 4, 6, 8, 1, 3, 5, 7]);
        x.rearrange(|&e| (e % 3) as u8);
        assert_eq!(x, vec![6, 3, 4, 1, 7, 2, 8, 5]);
    }

    #[test]
    fn rearrange_slice_test() {
        let x = &mut [1, 2, 3, 4, 5, 6, 7, 8];
        x.rearrange(|&e| (e % 2) as u8);
        assert_eq!(x, &mut [2, 4, 6, 8, 1, 3, 5, 7]);
        x.rearrange(|&e| (e % 3) as u8);
        assert_eq!(x, &mut [6, 3, 4, 1, 7, 2, 8, 5]);
    }

    #[test]
    fn sort_vec_test() {
        let mut rng = thread_rng();
        let mut x: Vec<i32> = vec![];
        for _ in 0..10000 { x.push(rng.gen()) }
        let mut y = x.clone();
        x.radix_sort();
        y.sort();
        assert_eq!(x, y);
    }

    #[test]
    fn sort_slice_test() {
        let mut rng = thread_rng();
        let mut x: Vec<i32> = vec![];
        for _ in 0..10000 { x.push(rng.gen()) }
        let mut y = x.clone();
        x.as_mut_slice().radix_sort();
        y.sort();
        assert_eq!(x, y);
    }
}


impl<X: RadixU8 + 'static, Y: RadixU8 + 'static> RadixU8 for (X, Y) {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        Y::do_radix_sort_with(a, |x| &(f(x).1));
        X::do_radix_sort_with(a, |x| &(f(x).0));
    }
}


impl<X: RadixU8 + 'static, Y: RadixU8 + 'static, Z: RadixU8+ 'static> RadixU8 for (X, Y, Z) {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        Z::do_radix_sort_with(a, |x| &(f(x).2));
        Y::do_radix_sort_with(a, |x| &(f(x).1));
        X::do_radix_sort_with(a, |x| &(f(x).0));
    }
}


impl RadixU8 for i64 {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        a.rearrange(|x| *f(x) as u8);
        a.rearrange(|x| (*f(x) >> 8) as u8);
        a.rearrange(|x| (*f(x) >> 16) as u8);
        a.rearrange(|x| (*f(x) >> 24) as u8);
        a.rearrange(|x| (*f(x) >> 32) as u8);
        a.rearrange(|x| (*f(x) >> 40) as u8);
        a.rearrange(|x| (*f(x) >> 48) as u8);
        a.rearrange(|x| ((*f(x) >> 56) ^ (1 << 7)) as u8); //flip the sign bit
    }
}

impl RadixU8 for i32 {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        a.rearrange(|x| *f(x) as u8);
        a.rearrange(|x| (*f(x) >> 8) as u8);
        a.rearrange(|x| (*f(x) >> 16) as u8);
        a.rearrange(|x| ((*f(x) >> 24) ^ (1 << 7)) as u8); //flip the sign bit
    }
}

impl RadixU8 for i16 {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        a.rearrange(|x| *f(x) as u8);
        a.rearrange(|x| ((*f(x) >> 8) ^ (1 << 7)) as u8); //flip the sign bit
    }
}

impl RadixU8 for i8 {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        a.rearrange(|x| (*f(x) ^ (1 << 7)) as u8); //flip the sign bit
    }
}


impl RadixU8 for u64 {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        a.rearrange(|x| *f(x) as u8);
        a.rearrange(|x| (*f(x) >> 8) as u8);
        a.rearrange(|x| (*f(x) >> 16) as u8);
        a.rearrange(|x| (*f(x) >> 24) as u8);
        a.rearrange(|x| (*f(x) >> 32) as u8);
        a.rearrange(|x| (*f(x) >> 40) as u8);
        a.rearrange(|x| (*f(x) >> 48) as u8);
        a.rearrange(|x| (*f(x) >> 56) as u8);
    }
}

impl RadixU8 for u32 {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        a.rearrange(|x| *f(x) as u8);
        a.rearrange(|x| (*f(x) >> 8) as u8);
        a.rearrange(|x| (*f(x) >> 16) as u8);
        a.rearrange(|x| (*f(x) >> 24) as u8);
    }
}

impl RadixU8 for u16 {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        a.rearrange(|x| *f(x) as u8);
        a.rearrange(|x| (*f(x) >> 8) as u8);
    }
}

impl RadixU8 for u8 {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        a.rearrange(|x| *f(x));
    }
}

impl RadixU8 for char {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        a.rearrange(|x| *f(x) as u32 as u8);
        a.rearrange(|x| (*f(x) as u32 >> 8) as u8);
        a.rearrange(|x| (*f(x) as u32 >> 16) as u8);
        a.rearrange(|x| (*f(x) as u32 >> 24) as u8);
    }
}

impl RadixU8 for usize {
    fn do_radix_sort_with<A: RadixSort + ?Sized, F>(a: &mut A, f: F) where F: Fn(&A::Item) -> &Self {
        a.rearrange(|x| *f(x) as u8);
        a.rearrange(|x| (*f(x) >> 8) as u8);
        a.rearrange(|x| (*f(x) >> 16) as u8);
        a.rearrange(|x| (*f(x) >> 24) as u8);
        a.rearrange(|x| (*f(x) >> 32) as u8);
        a.rearrange(|x| (*f(x) >> 40) as u8);
        a.rearrange(|x| (*f(x) >> 48) as u8);
        a.rearrange(|x| (*f(x) >> 56) as u8);
    }
}
