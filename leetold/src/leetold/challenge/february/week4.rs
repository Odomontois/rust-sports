pub fn score_of_parentheses(s: String) -> i32 {
    let mut stack = vec![0];
    for c in s.chars() {
        if c == '(' {
            stack.push(0);
            continue;
        }
        let res = 1.max(stack.pop().unwrap() * 2);
        *stack.last_mut().unwrap() += res
    }
    stack[0]
}
