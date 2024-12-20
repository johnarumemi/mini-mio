//! This module contains the FFI bindings for the `kqueue` API on BSD systems.
//!
//! Also contains filter flags and event flags for the `kevent` system call.
//!
//! # C function signature
//!
//! ```c
//! int kevent(
//!   int kq,
//!   const struct kevent *changelist,
//!   int nchanges,
//!   struct kevent *eventlist,
//!   int nevents,
//!   const struct timespec *timeout
//! );
//! ```
//!
//! # Other Resources
//!
//! - [MacOSX man page][1]
//! - [KernelQueues][2]
//! - [kevent][3]
//! - [FreeBSD man page][4]
//!
//! [1]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kqueue.2.html
//! [2]: https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/FSEvents_ProgGuide/KernelQueues/KernelQueues.html
//! [3]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kevent.2.html#//apple_ref/doc/man/2/kevent
//! [4]: https://man.freebsd.org/cgi/man.cgi?query=kevent&apropos=0&sektion=0&manpath=FreeBSD+15.0-CURRENT&arch=default&format=html
//! ---

use std::mem::MaybeUninit;
use std::ops::{BitAnd, BitOr};

use crate::sys::events::kqueue::OsEvent;

#[link(name = "c")] // link to libc
extern "C" {
    // use C calling convention

    /// `int kqueue(void)`
    pub(super) fn kqueue() -> i32;

    /// The kevent() system call returns the number of  events  placed  in  the
    /// eventlist, up to the value given by nevents. If an error occurs while
    /// processing an element of the changelist and there is enough room in the
    /// eventlist, then the event will be placed in the eventlist with EV_ERROR
    /// set in flags and the system error in `data`.  Otherwise, -1 will be
    /// returned, and errno will be set to indicate the error condition. If the
    /// time limit expires, then kevent() returns 0.
    ///
    /// ```c
    /// int kevent(
    ///   int kq,
    ///   const struct kevent *changelist,
    ///   int nchanges,
    ///   struct kevent *eventlist,
    ///   int nevents,
    ///   const struct timespec *timeout
    /// );
    /// ```
    pub(super) fn kevent(
        kqfd: i32,
        changelist: *const OsEvent,
        nchanges: i32,
        eventlist: *mut OsEvent,
        nevents: i32,
        timeout: *const timespec,
    ) -> i32;

    pub(super) fn close(fd: i32) -> i32;
}

// linux x32 compatibility
// See https://sourceware.org/bugzilla/show_bug.cgi?id=16437
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(super) struct timespec {
    pub tv_sec: i64, // time_t = c_long =  i64
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    pub tv_nsec: i64,
    #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
    pub tv_nsec: i64, // c_long = i64
}

impl From<Option<std::time::Duration>> for timespec {
    fn from(timeout: Option<std::time::Duration>) -> Self {
        match timeout {
            Some(duration) => timespec {
                tv_sec: duration.as_secs() as i64,
                tv_nsec: duration.subsec_nanos() as i64,
            },
            None => timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_timespec() {
        let duration = std::time::Duration::from_secs(5);

        let timespec = timespec::from(Some(duration));

        assert_eq!(timespec.tv_sec, 5);
        assert_eq!(timespec.tv_nsec, 0);
    }
}
