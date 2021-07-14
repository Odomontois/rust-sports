

pub fn number_of_steps(num: i32) -> i32 {
    (num.count_ones().max(1) - 1 + (32 - num.leading_zeros())) as i32
}

#[test]
fn number_test() {
    assert_eq!(number_of_steps(0), 0);
    assert_eq!(number_of_steps(1), 1);
    assert_eq!(number_of_steps(14), 6);
}

pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let mut group = vec![None; graph.len()];
    for start in 0..graph.len() {
        if group[start].is_some() {
            continue;
        }
        group[start] = Some(true);
        let mut stack = vec![(start, true)];
        while let Some((i, g)) = stack.pop() {
            for &j in &graph[i] {
                if let Some(b) = group[j as usize] {
                    if b == g {
                        return false;
                    }
                }
                stack.push((j as usize, !g));
                group[j as usize] = Some(!g);
            }
        }
    }
    true
}
