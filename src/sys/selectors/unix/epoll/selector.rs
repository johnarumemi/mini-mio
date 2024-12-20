//! Consider this the FFI for the epoll event queue
//!
//! This module contains the code related to the syscalls we
//! need to communicate with the host operating system.
//!
//! # FIXES:
//! The number is identical to the number in the GitHub issue tracker
//!
//! ## FIX ISSUE #5
//! See: https://github.com/PacktPublishing/Asynchronous-Programming-in-Rust/issues/5
//! Readers reported wrong results when running the example on ARM64 instruction set
//! (aarch64). The reason turned out to be that the `Event` struct is only `repr(packed)`
//! on `x86-64` systems due to backwards compatibility. Fixed by conditionally
//! compiling the #[repr(packed)] attribute.

use crate::sys::SysSelector;
use std::os::fd::OwnedFd;

/// Rather than the registry, the selector holds the raw file descriptor
pub struct Selector {
    id: OwnedFd,
}
