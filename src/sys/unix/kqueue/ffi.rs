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

use super::events::{timespec, Kevent};

#[link(name = "c")] // link to libc
extern "C" {
    // use C calling convention

    /// `int kqueue(void)`
    pub(super) fn kqueue() -> i32;

    /// The kevent() system call returns the number of  events  placed  in  the eventlist, up to
    /// the value given by nevents. If an error occurs while processing an element of the
    /// changelist and there is enough room in the eventlist, then the event will be placed in the
    /// eventlist with EV_ERROR set in flags and the system error in data.  Otherwise, -1 will be
    /// returned, and errno will be set to indicate the error condition. If the time limit expires,
    /// then kevent() returns 0.
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
        changelist: *const Kevent,
        nchanges: i32,
        eventlist: *mut Kevent,
        nevents: i32,
        timeout: *const timespec,
    ) -> i32;

    pub(super) fn close(fd: i32) -> i32;
}
