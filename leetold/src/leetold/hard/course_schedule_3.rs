use std::collections::BinaryHeap;

pub fn schedule_course<A: AsRef<[i32]>, B: AsRef<[A]>>(courses: B) -> i32 {
    let mut courses: Vec<_> = courses
        .as_ref()
        .iter()
        .map(|a| {
            let v = a.as_ref();
            [v[1], v[0]]
        })
        .collect();
    courses.sort();
    let mut cur = 0;
    let mut taken = BinaryHeap::<i32>::new();
    for [end, duration] in courses {
        if end - duration < cur {
            let largest = taken.peek().copied().unwrap_or(0);
            if largest >= cur - end + duration && largest > duration {
                taken.pop();
                cur -= largest;
            } else {
                continue;
            }
        }
        cur += duration;
        taken.push(duration)
    }
    taken.len() as i32
}

#[test]
fn example1() {
    assert_eq!(
        3,
        schedule_course([[100, 200], [200, 1300], [1000, 1250], [2000, 3200]])
    )
}

#[test]
fn example2() {
    assert_eq!(1, schedule_course([[1, 2]]))
}

#[test]
fn example3() {
    assert_eq!(0, schedule_course([[3, 2], [4, 3]]))
}

#[test]
fn wa1() {
    assert_eq!(
        5,
        schedule_course([[5, 15], [3, 19], [6, 7], [2, 10], [5, 16], [8, 14], [10, 11], [2, 19]])
    )
}

#[test]
fn wa2() {
    assert_eq!(3, schedule_course([[9, 14], [7, 12], [1, 11], [4, 7]]))
}
