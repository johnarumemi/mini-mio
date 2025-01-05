//! epoll based structures and types
use crate::interfaces::SysEvent;
use crate::sys::constants::epoll::events;

/// repr(packed) forces Rust to strip any padding, and only align the type to a byte
/// note: The OS syscall expects the struct we use to be packed.
///
/// # FIXES:
/// The number is identical to the number in the GitHub issue tracker
///
/// ## FIX ISSUE #5
/// See:
/// https://github.com/PacktPublishing/Asynchronous-Programming-in-Rust/issues/5
/// Readers reported wrong results when running the example on ARM64 instruction
/// set (aarch64). The reason turned out to be that the `Event` struct is only
/// `repr(packed)` on `x86-64` systems due to backwards compatibility with 32
/// bit arch syscalls. Fixed by conditionally compiling the #[repr(packed)]
/// attribute.
#[derive(Debug, Default)]
#[repr(C)]
#[cfg_attr(target_arch = "x86_64", repr(packed))] // only use packed on x86_64
pub struct OsEvent {
    // bitmask of events that we are interested in
    pub events: i32,

    // token to identify event source
    pub(crate) epoll_data: usize,
}

pub type OsEvents = Vec<OsEvent>;

impl SysEvent for OsEvent {
    fn token(&self) -> crate::interfaces::Token {
        crate::interfaces::Token(self.epoll_data)
    }

    /// Returns true if the event is readable
    fn is_readable(&self) -> bool {
        // ensure bitmask is set to ready for reading
        (self.events & events::EPOLLIN) != 0 || (self.events & events::EPOLLPRI) != 0
    }

    // return True if read direction of the socket has shut down
    fn is_read_closed(&self) -> bool {
        // Both halves of the socket have closed
        (self.events & events::EPOLLHUP != 0)
            // Socket has received FIN or called shutdown(SHUT_RD)
            || (self.events & events::EPOLLIN != 0 && self.events & events::EPOLLRDHUP != 0)
    }

    // return True if the event is writable
    fn is_writable(&self) -> bool {
        // ensure bitmask is set to ready for write operations
        (self.events & events::EPOLLOUT) != 0
    }

    /// return True if write direction of the socket has shut down
    /// 
    /// This can be due to following reasons:
    /// - Both halves of a socket have been closed
    /// - Unix pipe is readable, but error has occurred and write end is closed
    /// - Read end of a Unix pipe has closed (not readable)
    fn is_write_closed(&self) -> bool {
            // Both halves of the socket have closed
        (self.events & events::EPOLLHUP != 0) 
            // Unix pipe write end has closed (can still be read from)
            || (self.events & events::EPOLLOUT != 0 && self.events & events::EPOLLERR != 0)
            // The other side (read end) of a Unix pipe has closed.
            || self.events == events::EPOLLERR

    }

    fn is_error(&self) -> bool {
        (self.events & events::EPOLLERR) != 0
    }
}
