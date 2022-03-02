pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut acc = vec![];
    let mut start = 0;
    let mut cur = None::<i32>;
    let mut push = |oc: Option<i32>, os: Option<i32>| {
        if let Some(end) = oc {
            acc.push(if start == end {
                format!("{start}")
            } else {
                format!("{start}->{end}")
            });
        }
        for x in os {
            start = x
        }
    };
    for num in nums {
        push(cur.filter(|x| x + 1 < num), (cur < Some(num - 1)).then(|| num));
        cur = Some(num);
    }
    push(cur, None);
    acc
}

#[test]
fn test1() {
    assert_eq!(vec!["-1"], summary_ranges(vec![-1]))
}

#[test]
fn test2() {
    assert_eq!(vec!["0->2", "4->5", "7"], summary_ranges(vec![0, 1, 2, 4, 5, 7]));
}

#[test]
fn test3() {
    assert_eq!(
        vec!["0", "2->4", "6", "8->9"],
        summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
    )
}
