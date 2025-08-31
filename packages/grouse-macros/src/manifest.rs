use std::{
    collections::{HashSet, hash_set::Iter},
    path::Path,
};

use syn::LitStr;
use walkdir::WalkDir;

use crate::{cache::Cache, digest::Digest};

thread_local! {
    static CACHE: Cache = Cache::new();
}

pub fn resolve(path: &LitStr) -> std::io::Result<Option<Box<Path>>> {
    Ok(Some(
        std::env::current_dir()?
            .join(match path.span().unwrap().local_file() {
                Some(local_file) => local_file,
                None => return Ok(None),
            })
            .parent()
            .unwrap()
            .join(path.value())
            .canonicalize()?
            .into_boxed_path(),
    ))
}

pub fn digest(path: &LitStr) -> std::io::Result<Option<Digest>> {
    Ok(match resolve(path)? {
        Some(path) => {
            Some(CACHE.with(|cache| cache.get_or_try_set(path, |path| Digest::from_path(path)))?)
        }
        None => None,
    })
}

pub fn bundle(path: &LitStr) -> std::io::Result<Option<Manifest>> {
    let path = match resolve(path)? {
        Some(path) => path,
        None => return Ok(None),
    };

    let mut entries = HashSet::new();

    for entry in WalkDir::new(&path) {
        let entry = entry?;

        if entry.file_type().is_file() {
            entries.insert(Entry {
                digest: CACHE.with(|cache| {
                    cache.get_or_try_set(entry.path(), |path| Digest::from_path(path))
                })?,
                path: entry.into_path().into_boxed_path(),
            });
        }
    }

    Ok(Some(Manifest(entries)))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest(HashSet<Entry>);

impl Manifest {
    #[inline]
    pub fn entries(&self) -> Iter<'_, Entry> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entry {
    digest: Digest,
    path: Box<Path>,
}

impl Entry {
    #[inline]
    pub const fn digest(&self) -> Digest {
        self.digest
    }

    #[inline]
    pub const fn path(&self) -> &Path {
        &self.path
    }
}
