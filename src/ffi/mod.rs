pub mod epoll;

#[cfg(target_os = "macos")]
pub mod kqueue;
