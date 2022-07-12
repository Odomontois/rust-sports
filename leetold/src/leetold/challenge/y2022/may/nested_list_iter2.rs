use self::NestedInteger::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
#[derive(Default, Debug, Clone)]
struct NestedIterator {
    next: Option<i32>,
    stack: Vec<Vec<NestedInteger>>,
}

impl NestedIterator {
    fn pop_next(&mut self) {
        while let Some(mut v) = self.stack.pop() {
            let z = v.pop();
            if !v.is_empty() {
                self.stack.push(v)
            }
            match z {
                Some(Int(x)) => {
                    self.next = Some(x);
                    return;
                }
                Some(List(mut w)) => {
                    w.reverse();
                    self.stack.push(w)
                }
                None => {}
            }
        }
    }

    fn new(mut ni: Vec<NestedInteger>) -> Self {
        ni.reverse();
        let mut this = Self {
            stack: vec![ni],
            next: None,
        };
        this.pop_next();
        this
    }

    fn next(&mut self) -> i32 {
        let res = self.next.unwrap_or(-1);
        self.pop_next();
        res
    }

    fn has_next(&mut self) -> bool {
        self.next.is_some()
    }
}



#[cfg(test)]
mod test {

    #[test]
    fn test1() {}
}
