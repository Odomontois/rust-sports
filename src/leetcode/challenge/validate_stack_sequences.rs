pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = Vec::new();
    let mut pushed = pushed.into_iter();
    popped.iter().map(|p| -> Option<_>{
        while stack.last() != Some(p) {
            stack.push(pushed.next()?);
        }
        stack.pop().map(|_| ())
    }).collect::<Option<()>>().is_some()
}

