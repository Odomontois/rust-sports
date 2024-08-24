use std::mem::swap;

pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    radix_sort_i32(&mut nums);
    nums
}

fn radix_sort_i32(els: &mut [i32]) {
    let aux = &mut vec![0; els.len()];
    let f = |x: &i32| *x as usize;
    bucket_sort::<257, _>(els, |x| f(x) & 0xff, aux);
    bucket_sort::<257, _>(aux, |x| f(x) >> 8 & 0xff, els);
    bucket_sort::<257, _>(els, |x| f(x) >> 16 & 0xff, aux);
    bucket_sort::<257, _>(aux, |x| f(x) >> 24 & 0xff, els);
    bucket_sort::<3, _>(els, |x| (*x >= 0) as usize, aux);
    els.copy_from_slice(aux);
}

fn bucket_sort<const N: usize, A>(els: &mut [A], mut f: impl FnMut(&A) -> usize, aux: &mut [A]) {
    let mut buckets = [0; N];
    for el in &*els {
        buckets[f(el) as usize + 1] += 1;
    }
    for b in 1..N {
        buckets[b] += buckets[b - 1];
    }
    for i in 0..els.len() {
        let b = f(&els[i]) as usize;
        swap(&mut aux[buckets[b]], &mut els[i]);
        buckets[b] += 1;
    }
}

#[test]
fn calc() {
    use rand::RngCore;

    let mut x: Vec<_> = std::iter::repeat(rand::thread_rng().next_u32() as i32)
        .take(1000)
        .collect();
    // let mut x = vec![6, 4, 2, 7, 4, 2, -1, -3];
    radix_sort_i32(&mut x);
    for i in 1..x.len() {
        assert!(x[i - 1] <= x[i], "{} is greater than {}", x[i - 1], x[i]);
    }
}
