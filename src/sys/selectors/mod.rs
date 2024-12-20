//! This module contains the platform specific code implementing the `SysSelector` trait.
//!
//! It should expose the following type:
//! - `Selector`: used for interacting with the event queue. This will be

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::*;
