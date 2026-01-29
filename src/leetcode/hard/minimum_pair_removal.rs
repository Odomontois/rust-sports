pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    use std::collections::{BTreeSet, HashSet};

    let mut deviations = HashSet::new();
    let mut sums = BTreeSet::new();
    #[derive(Clone, Copy, Default)]
    struct Item {
        val: i64,
        prev: Option<usize>,
        next: Option<usize>,
    }
    let mut list = vec![Item::default(); nums.len()];
    list[0].val = nums[0] as i64;
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            deviations.insert(i);
        }
        sums.insert(((nums[i] + nums[i + 1]) as i64, i, i + 1));
        list[i + 1].val = nums[i + 1] as i64;
        list[i].next = Some(i + 1);
        list[i + 1].prev = Some(i);
    }
    let mut count = 0;
    while !deviations.is_empty() {
        count += 1;
        if let Some((s, i, j)) = sums.pop_first() {
            deviations.remove(&i);
            deviations.remove(&j);
            list[i].next = list[j].next;
            if let Some(k) = list[j].next {
                sums.remove(&(list[k].val + list[j].val, j, k));
                sums.insert((list[k].val + s, i, k));
                if s > list[k].val {
                    deviations.insert(i);
                }
                list[k].prev = Some(i);
            }
            if let Some(k) = list[i].prev {
                sums.remove(&(list[k].val + list[i].val, k, i));
                sums.insert((list[k].val + s, k, i));
                if list[k].val <= s {
                    deviations.remove(&k);
                } else {
                    deviations.insert(k);
                }
            }
            list[i].val = s;
        }
    }
    count
}

#[test]
fn example1() {
    assert_eq!(2, minimum_pair_removal(vec![5, 2, 3, 1]));
}

#[test]
fn example2() {
    assert_eq!(0, minimum_pair_removal(vec![1, 2, 2]));
}

#[test]
fn wa1() {
    assert_eq!(9, minimum_pair_removal(vec![2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1]));
}
