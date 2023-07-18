struct Card<I> {
    prev_element: I,
    next_element: I,
    prev_set: I,
    next_set: I,
    set: I,
    element: I,
    count: I,
}


#[test]
fn check() {
    println!("{}", std::mem::size_of::<Card<u16>>());
    println!("{}", std::mem::size_of::<Card<u32>>());
}