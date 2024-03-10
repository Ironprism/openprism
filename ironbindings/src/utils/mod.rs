use anyhow::{Context, Result};
use quote::ToTokens;
use std::hash::Hash;
use std::collections::HashMap;

/// read std::env::dir with an unique name
pub fn temp_dir() -> Result<std::path::PathBuf> {
    let mut path = std::env::temp_dir();
    path.push(std::path::Path::new(&format!(
        "irontraits-{}",
        uuid::Uuid::new_v4()
    )));
    log::debug!("Creating temp dir {}", path.to_string_lossy());
    std::fs::create_dir(&path)
        .with_context(|| format!("Could not create temp dir {}", path.to_string_lossy()))?;
    Ok(path)
}

pub struct Counter<T: Hash + Eq> {
    map: HashMap<T, usize>,
}

impl<T: Hash + Eq > Counter<T> {
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

pub trait TwinePush<T> {
    fn push(&mut self, item: T);
}

pub struct Twine<T> {
    items: Vec<Vec<T>>
}

impl<T> Twine<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new()
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        self.items.into_iter().flatten().collect()
    }
}

impl<T: ToTokens> ToTokens for Twine<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.items.iter().for_each(|sub_vec| {
            sub_vec.iter().for_each(|item| {
                item.to_tokens(tokens);
            });
        });
    }
}

impl<T> TwinePush<T> for Twine<T> {
    fn push(&mut self, item: T) {
        self.items.push(vec![item]);
    }
}

impl<T> TwinePush<Vec<T>> for Twine<T> {
    fn push(&mut self, item: Vec<T>) {
        self.items.push(item);
    }
}

impl<T> TwinePush<Twine<T>> for Twine<T> {
    fn push(&mut self, item: Twine<T>) {
        self.items.extend(item.items);
    }
}