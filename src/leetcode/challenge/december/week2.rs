pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut dd = vec![0; 60];
    for t in time { dd[(t % 60) as usize] += 1 };
    (dd[0] * (dd[0] - 1) / 2) + (dd[30] * (dd[30] - 1) / 2) + (1..30).map(|i| dd[i] * dd[60 - i]).sum::<i32>()
}

#[test]
fn test_num_pairs(){
    println!("{}", num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]));
}