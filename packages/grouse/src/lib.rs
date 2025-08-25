//! # grouse
//!
//! A simple asset bundler for Rust.
//!
//! ## about
//!
//! `grouse` is a very simple asset bundler intended for baking static web
//! content directly into your Rust binaries. This library can be thought of a more
//! specialized version of the fantastic `include_dir` crate; Whilst
//! `include_dir` provides a more traditional tree-like view of the embedded assets,
//! `grouse` uses `sha256` to generate a digest of each file and provides a flat,
//! hashmap-like view of digest, file pairs. This is particularly useful when
//! serving static web content, as changes to any files will effectively change the
//! name of that file being served, and invalidate any browser caches.
//!
//! ## getting started
//!
//! To start using `grouse`, you'll first need to add our package to your
//! `Cargo.toml` manifest:
//!
//! ```sh
//! cargo add grouse
//! ```
//!
//! Then you can bundle a directory into your Rust executable.
//!
//! ```rust
//! # use grouse::{Manifest, File};
//!
//! // Thanks to recently stablized proc-macro features, the include
//! // path is relative to the current file directory, like how the
//! // builtin [`core::include_bytes!`] macro works.
//! const MANIFEST: Manifest<'static> = grouse::include!("../src");
//!
//! fn main() {
//!   // Iterate through all of the files in the manifest.
//!   for file in MANIFEST {
//!      eprintln!("{} => {}", file.name(), file.digest());
//!   }
//!
//!   // We can also get the digest of a particular file.
//!   let x = grouse::digest!("../src/lib.rs");
//!
//!   // Which can then be used to lookup it's corresponding file in the
//!   // manifest.
//!   let lib = MANIFEST.get(x).unwrap();
//! }
//! ```

pub use grouse_macros::{digest, include};

/// A flat, hashmap-like container for all of the files included using
/// [`grouse::include!`]. This means that any subdirectories are flattened, and
/// all files are identified by their unique sha2 digest.
///
/// The path of each file is not included within this digest, which means that
/// any files with the exact same content, regardless of their subdirectory
/// or name, will be merged into a single file entry. In this case, the name
/// reported by [`File::name`] is undefined behaviour.
///
/// ## remarks
///
/// The fields of this struct are `pub` so that it can be initialized by the
/// [`grouse::include!`] macro, however, they are considered a private API for the
/// most part. Therefore, it is highly discouraged to directly access these fields
/// and their name must not be relied upon across versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Manifest<'a> {
    #[doc(hidden)]
    pub files: &'a [File<'a>],

    #[doc(hidden)]
    pub index: fn(&str) -> Option<&'a File<'a>>,
}

impl<'a> Manifest<'a> {
    /// An array of all the files in this [`Manifest`].
    #[inline]
    pub const fn files(&self) -> &'a [File<'a>] {
        &self.files
    }

    /// Lookup a [`File`] by it's digest.
    ///
    /// ## implementation
    ///
    /// Under the hood, this method calls a macro generated [`fn`] pointer
    /// that performs a `match` over the possible digests that this [`Manifest`] could
    /// contain. This is far more efficient than a real [`HashMap`] since we know all of
    /// the possible values at compile-time.
    ///
    /// If and when [`fn`] pointers get stablized as `const`, this function
    /// could also be marked as `const`.
    #[inline]
    pub fn get(&self, digest: &str) -> Option<&'a File<'a>> {
        (self.index)(digest)
    }
}

impl<'a> IntoIterator for Manifest<'a> {
    type Item = &'a File<'a>;

    type IntoIter = ::core::slice::Iter<'a, File<'a>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.files.iter()
    }
}

/// Represents a single file within a [`Manifest`].
///
/// ## remarks
///
/// The fields of this struct are `pub` so that it can be initialized by the
/// [`grouse::include!`] macro, however, they are considered a private API for the
/// most part. Therefore, it is highly discouraged to directly access these fields
/// and their name must not be relied upon across versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct File<'a> {
    #[doc(hidden)]
    pub bytes: &'a [u8],

    #[doc(hidden)]
    pub name: &'a str,

    #[doc(hidden)]
    pub digest: &'a str,

    #[doc(hidden)]
    pub mime: &'a str,
}

impl<'a> File<'a> {
    /// The raw content of the file, as bytes.
    #[inline]
    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// The original name of the file.
    ///
    /// ## remarks
    ///
    /// If multiple files in the included directory had the same digest, then
    /// this name could be any one of those.
    #[inline]
    pub const fn name(&self) -> &'a str {
        self.name
    }

    /// The `sha256` digest of the file's content.
    #[inline]
    pub const fn digest(&self) -> &'a str {
        self.digest
    }

    /// A best-effort guess of the mime type of this file.
    /// ## remarks
    ///
    /// If multiple files in the included directory had the same digest, then
    /// this mime could be any one of those.
    #[inline]
    pub const fn mime(&self) -> &'a str {
        self.mime
    }
}
