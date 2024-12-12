//! kqueue based structures and types

use std::ops::{BitAnd, BitOr};

/// Can also view a similar struct in the `libc` crate on ios/darwin
/// Does not need to be packed. It is 32 bytes anyway, with no padding.
#[derive(Debug)]
#[repr(C)]
pub struct Kevent {
    /// Typically the file descriptor we are
    /// interested in receiving notifications for.
    pub ident: usize,
    /// Identifies the kernel filter used to process this event.
    pub filter: i16, // aliased to i16
    /// Actions to perform on the event.
    pub flags: u16, // aliased to u16
    /// Filter flags.
    pub fflags: u32, // aliased to u32
    /// Aditional data passed to the filter.
    pub data: isize, // aliased to isize
    /// Opaque user data
    pub udata: usize, // *mut ::c_void,
}

// linux x32 compatibility
// See https://sourceware.org/bugzilla/show_bug.cgi?id=16437
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct timespec {
    pub tv_sec: i64, // time_t = c_long =  i64
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    pub tv_nsec: i64,
    #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
    pub tv_nsec: i64, // c_long = i64
}

/// predefined system filters
pub mod filters {
    /// returns whenever there is data available to read.
    pub const EVFILT_READ: i16 = -1;
    /// returns whenever it is possible to write to the file descriptor.
    pub const EVFILT_WRITE: i16 = -2;
    pub const EVFILT_AIO: i16 = -3;
    pub const EVFILT_VNODE: i16 = -4;
    pub const EVFILT_PROC: i16 = -5;
    pub const EVFILT_SIGNAL: i16 = -6;

    /// Establishes an interval timer identified by ident where
    /// data specifies the timeout period (in milliseconds).
    ///
    /// `fflags` can include one of the following flags to specify a different unit:
    /// - NOTE_SECONDS   data is in seconds
    /// - NOTE_USECONDS  data is in microseconds
    /// - NOTE_NSECONDS  data is in nanoseconds
    pub const EVFILT_TIMER: i16 = -7;
    pub const EVFILT_MACHPORT: i16 = -8;
    pub const EVFILT_FS: i16 = -9;
    pub const EVFILT_USER: i16 = -10;
    pub const EVFILT_VM: i16 = -12;
}

/// Actions to perform on the event.
pub mod flags {
    ///  Adds the event to the kqueue. Also automatically enables (EV_ENABLE).
    pub const EV_ADD: u16 = 0x1;
    /// Remove the event from the kqueue. Events which are attached to file descriptors are
    /// automatically deleted on the last close of the descriptor.
    pub const EV_DELETE: u16 = 0x2;
    /// Permit kevent() to return the event if it is triggered
    pub const EV_ENABLE: u16 = 0x4;
    /// Disable  the  event so  kevent() will not return it
    pub const EV_DISABLE: u16 = 0x8;
    /// Deliver one event, then disable the event
    pub const EV_ONESHOT: u16 = 0x10;
    /// After the event is retrieved by the user, its state is reset. This is useful for filters
    /// which report state transitions instead of the current state. Note that some filters may
    /// automatically set this flag internally.
    pub const EV_CLEAR: u16 = 0x20;
    /// Causes kevent() to return with EV_ERROR set without draining any pending events after
    /// updating events in the kqueue. When a filter is successfully added, the data field will be
    /// zero. This flag is useful for making bulk changes to a kqueue.
    pub const EV_RECEIPT: u16 = 0x40;
    /// Disable the event source immediately after delivery of an event. See EV_DISABLE above.
    pub const EV_DISPATCH: u16 = 0x80;
    pub const EV_FLAG0: u16 = 0x1000;
    pub const EV_POLL: u16 = 0x1000;
    pub const EV_FLAG1: u16 = 0x2000;
    pub const EV_OOBAND: u16 = 0x2000;
    pub const EV_ERROR: u16 = 0x4000;
    /// Filters may set this flag to indicate filter-specific EOF condition.
    pub const EV_EOF: u16 = 0x8000;
    pub const EV_SYSFLAGS: u16 = 0xf000;
}

pub mod fflags {
    pub const NOTE_TRIGGER: u32 = 0x01000000;
    pub const NOTE_FFNOP: u32 = 0x00000000;
    pub const NOTE_FFAND: u32 = 0x40000000;
    pub const NOTE_FFOR: u32 = 0x80000000;
    pub const NOTE_FFCOPY: u32 = 0xc0000000;
    pub const NOTE_FFCTRLMASK: u32 = 0xc0000000;
    pub const NOTE_FFLAGSMASK: u32 = 0x00ffffff;
    pub const NOTE_LOWAT: u32 = 0x00000001;
    pub const NOTE_DELETE: u32 = 0x00000001;
    pub const NOTE_WRITE: u32 = 0x00000002;
    pub const NOTE_EXTEND: u32 = 0x00000004;
    pub const NOTE_ATTRIB: u32 = 0x00000008;
    pub const NOTE_LINK: u32 = 0x00000010;
    pub const NOTE_RENAME: u32 = 0x00000020;
    pub const NOTE_REVOKE: u32 = 0x00000040;
    pub const NOTE_NONE: u32 = 0x00000080;
    pub const NOTE_EXIT: u32 = 0x80000000;
    pub const NOTE_FORK: u32 = 0x40000000;
    pub const NOTE_EXEC: u32 = 0x20000000;
}
