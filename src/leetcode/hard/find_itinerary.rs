pub fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
    use std::{convert::TryInto, collections::HashMap};

    let mut graph = HashMap::<String, Vec<String>>::new();
    tickets.sort_unstable_by(|v1, v2| v2.iter().cmp(v1.iter()));
    for v in tickets {
        if let Ok::<[String; 2], _>([s, t]) = v.try_into() {
            graph.entry(s).or_default().push(t);
        }
    }
    let mut res = Vec::new();
    let mut stack = vec!["JFK".to_string()];
    while let Some(top) = stack.last() {
        if let Some(next) = graph.get_mut(top).and_then(|v| v.pop()) {
            stack.push(next);
        } else if let Some(next) = stack.pop() {
            res.push(next);
        }
    }
    res.reverse();
    res
}
