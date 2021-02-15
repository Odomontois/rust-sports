use crate::leetcode::data::{Tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Tree) -> String {
        let mut stack = vec![Some(root)];
        let mut res = String::new();
        while let Some(el) = stack.pop() {
            if let Some(Some(rc)) = el {
                res.push_str("[ ");
                let node = rc.borrow();
                res.push_str(&format!("{} ", node.val));
                stack.push(None);
                stack.push(Some(node.right.clone()));
                stack.push(Some(node.left.clone()));
            } else if let Some(None) = el {
                res.push_str("[] ")
            } else {
                res.push_str("] ")
            }
        }
        res
    }

    fn deserialize(&self, data: String) -> Tree {
        let mut state = Deserialize::new();

        data.split_ascii_whitespace()
            .try_for_each(|t| state.feed(t))
            .unwrap();
        state.result().unwrap()
    }
}

struct Deserialize {
    stack: Vec<(Place, TreeNode)>,
    res: Option<Tree>,
}

impl Deserialize {
    fn new() -> Self {
        Deserialize {
            stack: Vec::new(),
            res: None,
        }
    }

    fn feed_new_tree(&mut self, token: &str) -> Result<(), String> {
        match token {
            "[" => Ok(self.stack.push((
                Place::Val,
                TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                },
            ))),
            "[]" => self.feed_top(None),
            _ => Err(format!("expected `[` or `[]` , got `{}`", token)),
        }
    }

    fn feed_top(&mut self, tree: Tree) -> Result<(), String> {
        let (p, t) = match self.stack.last_mut() {
            Some(l) => l,
            None => {
                self.res = Some(tree);
                return Ok(());
            }
        };
        let v = match p {
            Place::Left => {
                *p = Place::Right;
                &mut t.left
            }
            Place::Right => {
                *p = Place::End;
                &mut t.right
            }
            _ => return Err(format!("unexpected error")),
        };
        Ok(*v = tree)
    }
    fn feed(&mut self, token: &str) -> Result<(), String> {
        let (p, t) = match self.stack.last_mut() {
            Some(l) => l,
            None => return self.feed_new_tree(token),
        };
        if *p == Place::Val {
            t.val = token.parse().map_err(|e| format!("{}", e))?;
            *p = Place::Left;
            return Ok(());
        } else if *p == Place::End {
            if token != "]" {
                return Err(format!("expected `]` got {}", token));
            }
            let (_, n) = self.stack.pop().unwrap();
            return self.feed_top(Some(Rc::new(RefCell::new(n))));
        }
        self.feed_new_tree(token)
    }
    fn result(self) -> Result<Tree, String> {
        self.res.ok_or(format!("unexpected end of input"))
    }
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Place {
    Val,
    Left,
    Right,
    End,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leetcode::data::tree;

    #[test]
    fn ser_test() {
        let t = tree(
            1,
            tree(2, tree(3, None, None), None),
            tree(4, None, tree(5, None, None)),
        );
        println!("{}", Codec::new().serialize(t));
    }

    #[test]
    fn deser_test() {
        println!("{:?}", Codec::new().deserialize(format!("[]")));
        println!(
            "{:?}",
            Codec::new().deserialize(format!("[ 1 [ 2 [ 3 [] [] ] [] ] [ 4 [] [ 5 [] [] ] ] ] "))
        );
    }
}
