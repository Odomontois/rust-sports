pub fn can_form_array(arr: Vec<i32>, mut pieces: Vec<Vec<i32>>) -> bool {
    let mut rev: Vec<Option<usize>> = vec![None; 101];
    for (i, &a) in arr.iter().enumerate() {
        rev[a as usize] = Some(i);
    }
    pieces.sort_by_key(|v| rev[v[0] as usize]);
    pieces.concat() == arr
}