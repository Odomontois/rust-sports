pub fn beautiful_array(n: i32) -> Vec<i32> {
    let k = n.leading_zeros();
    let trans = |x: u32| (x.reverse_bits() >> k) as i32;
    (1..1 << (32 - k)).map(trans).filter(|&x| x <= n).collect()
}
#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn hello() {
        fn check(n: i32) { 
            let xs = beautiful_array(n);
            let non_beauty = xs.iter().combinations(3).filter(|v| v[0] + v[2] == 2 * v[1]).next();
            assert_eq!(non_beauty, None, "n = {} result = {:?}", n, xs);
        }

        for i in 1..60 {
            check(i)
        }
    }
}
