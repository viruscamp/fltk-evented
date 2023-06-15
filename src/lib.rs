#![doc = include_str!("../README.md")]

#![cfg_attr(feature = "tokio", doc = concat!(r##"
## Async usage
This crate provides an AsyncListener which can be used in async contexts. This requires enabling either the tokio or async-std features. You can check the examples directory for an example on usage.

```rust
"##,
include_str!("../examples/tokio_ex/src/main.rs"),
r##"
```
"##))]

#![allow(clippy::needless_doctest_main)]

pub mod base;

pub mod blocking;
pub use blocking::Listener;

#[cfg(all(feature = "tokio", feature = "async-std"))]
compile_error!("Features `tokio` and `sync-std` are mutually exclusive.");

#[cfg(any(feature = "tokio", feature = "async-std"))]
pub mod asynch;
#[cfg(any(feature = "tokio", feature = "async-std"))]
pub use asynch::AsyncListener;
