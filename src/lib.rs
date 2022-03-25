//! Frequently used javascript functions for `Rust` and `WebAssembly`
//!
//! `webru` is a binding for [web-sys](https://crates.io/crates/web-sys)
//!
//! This crate assumes that you will only use this crate inside browser. Not any other javascript runtime such as `Node.js`
//!

mod global;
mod selectors;
mod timer;

// exporting functions

pub use global::*;
pub use selectors::*;
pub use timer::*;
