pub fn minimum_boxes(n: i32) -> i32 {
    let n = n as u64;
    let (mut low, mut high) = (0, n + 1);
    while high - low > 1 {
        let m = (low + high) / 2;
        let b = box_count(m);
        if b == n {
            return m as i32;
        } else if b > n {
            high = m
        } else {
            low = m
        }
    }
    high as i32
}

fn box_count(bottom: u64) -> u64 {
    if bottom == 0 {
        return 0;
    }
    let n = (((1 + 8 * bottom) as f64).sqrt() as u64 - 1) / 2;
    let tri = n * (n + 1) / 2;
    let rem = bottom - tri;
    bottom + box_count(n * (n - 1) / 2 + rem.max(1) - 1)
}

#[test]
fn count_test() {
    assert_eq!(box_count(6), 10);
    assert_eq!(box_count(5), 7);
    assert_eq!(box_count(4), 5);
    assert_eq!(box_count(3), 4);
    assert_eq!(box_count(2), 2);
}

#[test]
fn boxes_test() {
    assert_eq!(minimum_boxes(3), 3);
    assert_eq!(minimum_boxes(4), 3);
    assert_eq!(minimum_boxes(10), 6);
}
