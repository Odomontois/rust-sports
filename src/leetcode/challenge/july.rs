pub fn custom_sort_string(order: String, str: String) -> String {
    let mut x: Vec<_> = str.chars().collect();
    x.sort_by_key(|c| order.chars().position(|x| &x == c));
    x.into_iter().collect()
}
