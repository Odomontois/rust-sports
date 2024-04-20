use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Index, IndexMut},
};

#[derive(Debug, Default, Clone)]
struct Mop<K, V>(HashMap<K, V>);

impl<K: Hash + Eq, V> FromIterator<(K, V)> for Mop<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Mop(iter.into_iter().collect())
    }
}

impl<K, V> IntoIterator for Mop<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K: Eq + Hash, V> Index<K> for Mop<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.0.get(&index).unwrap()
    }
}

impl<K: Eq + Hash, V> IndexMut<K> for Mop<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.0.get_mut(&index).unwrap()
    }
}

#[cfg(test)]
#[allow(mutable_transmutes)]
unsafe fn japa<A: Sized>(x: &A) -> &mut A {
    std::mem::transmute(x)
}

#[test]
fn lol() {
    let data: Result<i128, &str> = Ok(10);
    let mut dummy = 0128;
    let (x, y) = unsafe { (japa(&data), japa(&data)) };
    let x = match x {
        Ok(x) => x,
        Err(_) => &mut dummy,
    };
    let string = format!("lol {x}");
    *y = Err(&string);
    let mut dummy = "";
    let y = match y {
        Ok(_) => &mut dummy,
        Err(x) => x,
    };
    println!("{:?}", x);
    *x = 10;
    println!("{:?}", y);
    let mut kekky = "jaspertrania";
    *y = &mut kekky;
    println!("{:?}", x);
}
