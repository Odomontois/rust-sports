pub fn beautiful_array(n: i32) -> Vec<i32> {
    let mut xs: Vec<_> = (1..=n).collect();
    xs.sort_by_key(|&x| x.reverse_bits());
    xs
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

        for i in 23..40 {
            check(i)
        }
    }
}
