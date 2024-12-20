//! kqueue based structures and types
use crate::interfaces::SysEvent;
use std::ops::{BitAnd, BitOr};

use crate::sys::constants::kqueue::{filters, flags};

/// Kevent is the event structure used by kqueue.
///
/// Can also view a similar struct in the `libc` crate on ios/darwin
/// Does not need to be packed. It is 32 bytes anyway, with no padding.
#[derive(Debug, Default)]
#[repr(C)]
pub struct OsEvent {
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

pub type OsEvents = Vec<OsEvent>;

/// Filters place an event on the kqueue for the user to retrieve, hence
/// the filter should be checked on the event to determine what sort of event has occurred.
///
/// Filters also set the `flag` field to indicate possible errors.
impl SysEvent for OsEvent {
    fn token(&self) -> crate::interfaces::Token {
        crate::interfaces::Token(self.udata)
    }

    /// Returns true if the event is readable
    fn is_readable(&self) -> bool {
        self.filter == filters::EVFILT_READ
    }

    // return True if read direction of the socket has shut down
    fn is_read_closed(&self) -> bool {
        // If the read direction of	the socket  has	 shut-
        // down,  then  the	 filter	 also  sets  EV_EOF in
        // flags, and returns the socket error (if any) in
        // fflags.	It is possible for EOF to be  returned
        // (indicating the connection is gone) while there
        // is still	data pending in	the socket buffer.
        (self.filter == filters::EVFILT_READ) && (self.flags & flags::EV_EOF != 0)
    }

    // return True if the event is writable
    fn is_writable(&self) -> bool {
        self.filter == filters::EVFILT_WRITE
    }

    // return True if write direction of the socket has shut down
    fn is_write_closed(&self) -> bool {
        // Write is closed if filter type is `EVFILT_WRITE`
        // and EV_EOF flag is set on the event.
        (self.filter == filters::EVFILT_WRITE) && (self.flags & flags::EV_EOF != 0)
    }

    fn is_error(&self) -> bool {
        // Filters can also set flags on an event to specify that an error has occurred.
        (self.flags & flags::EV_ERROR) != 0 || (self.flags & flags::EV_EOF != 0)
    }
}
