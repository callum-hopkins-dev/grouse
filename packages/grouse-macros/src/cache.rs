use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::digest::Digest;

pub struct Cache(RefCell<HashMap<PathBuf, Digest>>);

impl Cache {
    #[inline]
    pub fn new() -> Self {
        Self(RefCell::new(HashMap::new()))
    }

    pub fn get_or_try_set<K, F, E>(&self, k: K, f: F) -> Result<Digest, E>
    where
        K: AsRef<Path>,
        F: FnOnce(&Path) -> Result<Digest, E>,
    {
        match self.0.borrow_mut().entry(k.as_ref().to_owned()) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                Ok(*occupied_entry.get())
            }

            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                let x = (f)(vacant_entry.key())?;
                Ok(*vacant_entry.insert(x))
            }
        }
    }
}
