//! kqueue implementation for Unix systems.
//!
//! Exports following types:
//! - `Selector`
#![allow(unused, dead_code)]
mod ffi;
mod selector;

pub use selector::Selector;
