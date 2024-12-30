//! DEPRECATED: This module is deprecated and will be removed in the future.
pub mod epoll;

#[cfg(target_os = "macos")]
pub mod kqueue;
