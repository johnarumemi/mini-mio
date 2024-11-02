//! This module contains the FFI bindings for the `kqueue` API on BSD systems.
//!
//! MacOSX man page
//! https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kqueue.2.html
//! https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/FSEvents_ProgGuide/KernelQueues/KernelQueues.html
//!
//! kevent: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kevent.2.html#//apple_ref/doc/man/2/kevent
//!
//! FreeBSD man page
//! https://man.freebsd.org/cgi/man.cgi?query=kevent&apropos=0&sektion=0&manpath=FreeBSD+15.0-CURRENT&arch=default&format=html
//!
#![allow(unused, dead_code)]
mod events;

pub use events::*;

// use libc::{c_char, c_void, mmap, off_t, size_t, EVFILT_READ};
// use libc::{c_int, O_CREAT, O_EVTONLY, O_EXCL, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY};
// use stdint::{intptr_t, uintptr_t};

// use std::{fs::File, os::unix::io::FromRawFd};

use std::ops::{BitAnd, BitOr};

#[link(name = "c")] // link to libc
extern "C" {
    // use C calling convention

    /// `int kqueue(void)`
    pub fn kqueue() -> i32;

    /// The kevent() system call returns the number of  events  placed  in  the eventlist, up to the
    /// value given by nevents. If an error occurs while processing an element of the changelist
    /// and there is enough room in the eventlist, then the event will be placed in the eventlist
    /// with EV_ERROR set in flags and the system error in data.  Otherwise, -1 will be  returned,
    /// and errno will be set to indicate the error condition. If the time limit expires, then
    /// kevent() returns 0.
    pub fn kevent(
        kqfd: i32,
        changelist: *const Kevent,
        nchanges: i32,
        eventlist: *mut Kevent,
        nevents: i32,
        timeout: *const timespec,
    ) -> i32;

    pub fn close(fd: i32) -> i32;
}

/*
   @code c
   int kevent(
     int kq,
     const struct kevent *changelist,
     int nchanges,
     struct kevent *eventlist,
     int nevents,
     const struct timespec *timeout
   );
   @end

    struct kevent {
      uintptr_t  ident;       /* identifier for this event */
      short	 filter;       /* filter for event */
      u_short	 flags;	       /* action flags for kqueue */
      u_int	 fflags;       /* filter flag value */
      int64_t	 data;	       /* filter data value */
      void	 *udata;       /* opaque user data identifier */
      uint64_t	 ext[4];       /* extensions */
    };

*/
