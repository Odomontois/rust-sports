pub fn number_of_steps (num: i32) -> i32 {
    (num.count_ones().max(1) - 1 + (32 - num.leading_zeros())) as i32
}

#[test]
fn number_test(){
    assert_eq!(number_of_steps(0), 0);
    assert_eq!(number_of_steps(1), 1);
    assert_eq!(number_of_steps(14), 6);
}