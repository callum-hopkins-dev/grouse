<div align="center">

# grouse

A simple asset bundler for Rust.

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/callum-hopkins-dev/grouse/build.yaml?branch=main&event=push&style=for-the-badge)](https://github.com/callum-hopkins-dev/grouse/actions/workflows/build.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/grouse?style=for-the-badge)](https://crates.io/crates/grouse)
[![docs.rs](https://img.shields.io/docsrs/grouse?style=for-the-badge)](https://docs.rs/grouse/latest/grouse)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/grouse?style=for-the-badge)](https://crates.io/crates/grouse)
[![GitHub License](https://img.shields.io/github/license/callum-hopkins-dev/grouse?style=for-the-badge)](https://github.com/callum-hopkins-dev/grouse/blob/main/LICENSE)

</div>

## about

`grouse` is a very simple asset bundler intended for baking static web content
directly into your Rust binaries. This library can be thought of a more
specialized version of the fantastic `include_dir` crate; Whilst `include_dir`
provides a more traditional tree-like view of the embedded assets, `grouse` uses
`sha256` to generate a digest of each file and provides a flat, hashmap-like
view of digest, file pairs. This is particularly useful when serving static web
content, as changes to any files will effectively change the name of that file
being served, and invalidate any browser caches.

## getting started

To start using `grouse`, you'll first need to add our package to your
`Cargo.toml` manifest:

```sh
cargo add grouse
```

Then you can bundle a directory into your Rust executable.

```rust
// Thanks to recently stablized proc-macro features, the include
// path is relative to the current file directory, like how the
// builtin [`core::include_bytes!`] macro works.
const MANIFEST: Manifest<'static> = grouse::include!("../src");

fn main() {
  // Iterate through all of the files in the manifest.
  for file in MANIFEST {
     eprintln!("{} => {}", file.name(), file.digest());
  }

  // We can also get the digest of a particular file.
  let x = grouse::digest!("../src/lib.rs");

  // Which can then be used to lookup it's corresponding file in the
  // manifest.
  let lib = MANIFEST.get(x).unwrap();
}
```
