//! OS specific types
//!
//! These will be conditionally compiled based on OS. Only
//! one of these flags should hold true. Importing from
//! these modules should give following types within this modules namespace:
//!
//! - `Event`: alias for the OS event queue type that holds the event
//!     - Linux: epoll_event
//!     - MacOS: kevent
//! - `Events`: a collection of "`Event`"s
//! - `Selector`: used for interacting with the event queue. This will be
//!    used by the Registry for executing the lower level OS specific syscalls.

#[allow(unused_imports)]
pub(crate) mod constants;

#[allow(unused_imports)]
mod events;

#[allow(unused_imports)]
pub use events::{OsEvent, OsEvents};

#[allow(unused_imports)]
pub mod selectors;
