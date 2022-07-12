pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = vec![];
    let mut pushed = pushed.into_iter();
    for x in popped {
        while stack.last() != Some(&x) {
            if let Some(y) = pushed.next() {
                stack.push(y)
            } else {
                return false;
            }
        }
        stack.pop();
    }
    true
}
