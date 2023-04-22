pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    popped.iter().scan((vec![], pushed.into_iter()), |(stack, pushed), p| -> Option<_>{
        while stack.last() != Some(p) {
            stack.push(pushed.next()?);
        }
        Some(stack.pop().map(|_| ()))
    }).collect::<Option<()>>().is_some()
}

