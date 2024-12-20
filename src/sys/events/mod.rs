//! OS event queue data structures
//!
//! It should expose the following types:
//! - `OsEvent`: alias for the OS event queue type that holds the event
//!     - Linux: epoll_event
//!     - MacOS: kevent
//! - `OsEvents`: a collection of "`Event`"s
#[cfg(target_os = "macos")]
pub mod kqueue;

#[cfg(target_os = "macos")]
pub use kqueue::{OsEvent, OsEvents};
