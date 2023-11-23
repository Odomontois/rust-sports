pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
    pairs.sort();

    let mut stack = Vec::<i32>::new();
    for p in pairs {
        let (x, y) = (p[0], p[1]);
        let i = stack.binary_search(&x).map_or_else(|x| x, |x| x);
        if i == stack.len() {
            stack.push(y);
        } else if stack[i] > y {
            stack[i] = y;
        }
    }
    stack.len() as i32
}
