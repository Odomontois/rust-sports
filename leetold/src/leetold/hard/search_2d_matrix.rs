pub fn search_matrix<A: AsRef<[i32]>>(matrix: Vec<A>, target: i32) -> bool {
    matrix.iter().any(|v| v.as_ref().binary_search(&target).is_ok())
}
