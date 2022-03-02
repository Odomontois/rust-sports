
pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    left & ((!0)
        ^ (0..32)
            .rev()
            .map(|i| 1 << i)
            .find(|&b| left & b != right & b)
            .map(|t| (((t as u32) << 1) - 1) as i32)
            .unwrap_or(0))
}

#[test]
fn test1(){
    assert_eq!(4, range_bitwise_and(5, 7))
}
#[test]
fn test2(){
    assert_eq!(0, range_bitwise_and(0, 0))
}

#[test]
fn test3(){
    assert_eq!(0, range_bitwise_and(1, 2147483647))
}
