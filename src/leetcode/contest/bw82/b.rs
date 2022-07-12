pub fn latest_time_catch_the_bus(mut buses: Vec<i32>, mut passengers: Vec<i32>, capacity: i32) -> i32 {
    buses.sort();
    passengers.sort();
    let mut p = 0;
    let mut t = 0;
    for b in buses {
        let mut cap = capacity;
        while p < passengers.len() && passengers[p] <= b && cap > 0 {
            if p == 0 || passengers[p - 1] < passengers[p] - 1 {
                t = passengers[p] - 1;
            }
            cap -= 1;
            p += 1;
        }
        if cap > 0 && (p == 0 || passengers[p - 1] < b) {
            t = b
        }
    }
    t
}

#[test]
fn test1() {
    assert_eq!(16, latest_time_catch_the_bus(vec![10, 20], vec![2, 17, 18, 19], 2))
}

#[test]
fn test2() {
    assert_eq!(3, latest_time_catch_the_bus(vec![3], vec![2, 4], 2))
}

#[test]
fn test3() {
    assert_eq!(1, latest_time_catch_the_bus(vec![2], vec![2, 3], 2))
}
