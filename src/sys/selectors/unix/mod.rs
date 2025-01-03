//! This module contains the platform specific code for Unix systems.

#[cfg(target_os = "linux")]
mod epoll;

#[cfg(target_os = "linux")]
#[allow(unused_imports, dead_code)]
pub use epoll::*;

#[cfg(target_os = "macos")]
mod kqueue;

#[cfg(target_os = "macos")]
#[allow(unused_imports, dead_code)]
pub use kqueue::*;
