pub fn simplify_path(path: String) -> String {
    let mut dirs = vec![];
    for sub in path.split("/") {
        match sub {
            ".." => drop(dirs.pop()),
            "." | "" => {}
            s => dirs.push(s),
        }
    }
    if dirs.is_empty() {
        return "/".to_string();
    }
    dirs.into_iter().flat_map(|s| "/".chars().chain(s.chars())).collect()
}
