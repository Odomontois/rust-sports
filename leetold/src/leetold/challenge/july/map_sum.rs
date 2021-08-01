use std::collections::HashMap;

#[derive(Default)]
struct MapSum(HashMap<String, i32>);


impl MapSum {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, key: String, val: i32) {
        self.0.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        self.0.iter().filter(|(k, _)| k.starts_with(&prefix)).map(|(_, v)| v).sum()
    }
}
