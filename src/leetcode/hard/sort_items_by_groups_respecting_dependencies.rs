#[derive(Debug, Default, Clone)]
struct Restrict {
    before: usize,
    after: Vec<usize>,
}

pub fn sort_items<V: AsRef<[i32]>>(n: i32, m: i32, group: Vec<i32>, before_items: Vec<V>) -> Vec<i32> {
    let n = n as usize;
    let m = m as usize;

    let mut elements = vec![Restrict::default(); n + m * 2];
    let mut reg = |i: usize, j: usize| {
        elements[i].after.push(j);
        elements[j].before += 1;
    };
    for i in 0..n {
        let before = before_items[i].as_ref().iter().map(|&j| j as usize);
        if group[i] >= 0 {
            let gstart = group[i] as usize + n;
            let gend = gstart + m;
            reg(gstart, i);
            reg(i, gend);
            for j in before.clone() {
                if group[j] != group[i] {
                    if group[j] >= 0 {
                        reg(group[j] as usize + n + m, gstart);
                    } else {
                        reg(j, gstart);
                    }
                }
            }
        }
        for j in before {
            reg(j, i);
        }
    }

    let mut ready: Vec<_> = (0..2 * m + n).filter(|&i| elements[i].before == 0).collect();

    let mut res: Vec<i32> = Vec::<i32>::with_capacity(n);
    while let Some(i) = ready.pop() {
        if i < n {
            res.push(i as i32);
        }
        for a in std::mem::take(&mut elements[i].after) {
            elements[a].before -= 1;
            if elements[a].before == 0 {
                ready.push(a);
            }
        }
    }

    if res.len() == n {
        res
    } else {
        vec![]
    }
}

#[test]
fn example1() {
    let res = sort_items(
        8,
        2,
        vec![-1, -1, 1, 0, 0, 1, 0, -1],
        vec![&[] as &[i32], &[6], &[5], &[6], &[3, 6], &[], &[], &[]],
    );
    assert_eq!(8, res.len());
    println!("{res:?}");
}
