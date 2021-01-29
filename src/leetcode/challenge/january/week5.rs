use std::collections::BTreeMap;

use crate::leetcode::data::Tree;

pub fn vertical_traversal(root: Tree) -> Vec<Vec<i32>> {
    let mut vs = BTreeMap::new();
    v_traverse(root, 0, 0, &mut vs);
    vs.into_iter().map(|(_, mut v)| {
        v.sort();
        v.into_iter().map(|(_, v)| v).collect()
    }).collect()
}

fn v_traverse(root: Tree, x: i32, y: i32, vs: &mut BTreeMap<i32, Vec<(i32, i32)>>) -> Option<()> {
    let v = root?;
    let r = v.borrow();
    vs.entry(x).or_insert(vec![]).push((y, r.val));
    v_traverse(r.left.clone(), x - 1, y + 1, vs);
    v_traverse(r.right.clone(), x + 1, y + 1, vs);
    Some(())
}

async fn do_thing_1() {}

fn do_thing_2() -> Result<(), &'static str> {
    Ok(())
}

fn do_thing_3() -> Result<(), &'static str> {
    Ok(())
}

// async fn get_sum(maybe_x: Option<i32>, maybe_y: Option<i32>) -> Result<i32, &'static str> {
//     let x = maybe_x.into_result().or_else(|| {
//         do_thing_1().await;
//         do_thing_2()?;
//         return Err("Bad x");
//     });
//     let y = if let Some(y) = maybe_y {
//         y
//     } else {
//         do_thing_3()?;
//         return Err("This time something wrong with y");
//     };
//     return Ok(x + y);
// }