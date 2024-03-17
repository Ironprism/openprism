use std::collections::HashMap;
use std::hash::Hash;

pub struct Counter<T: Hash + Eq> {
    map: HashMap<T, usize>,
}

impl<T: Hash + Eq> Counter<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: T) {
        *self.map.entry(key).or_default() += 1;
    }

    pub fn get(&self, key: &T) -> Option<&usize> {
        self.map.get(key)
    }
}

impl<T: Hash + Eq> IntoIterator for Counter<T> {
    type Item = (T, usize);
    type IntoIter = std::collections::hash_map::IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}