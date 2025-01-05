//! Consider this the FFI for the epoll event queue
//!
//! This module contains the code related to the syscalls we
//! need to communicate with the host operating system.
//!

use std::io;
use std::mem::MaybeUninit;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};

use crate::interests::Interest;
use crate::interfaces::{SysSelector, Token};

// types used for interfacing with epoll syscalls
use crate::sys::constants::epoll::{events, ops};
use crate::sys::events::{OsEvent, OsEvents};

// epoll syscalls
use super::ffi;

/// Rather than the registry, the selector holds the raw file descriptor
pub struct Selector {
    /// OwnedFd is a wrapper around an i32.
    /// It closes the file descriptor when dropped: no need for `close` syscall.
    epfd: OwnedFd,
}

impl SysSelector for Selector {
    type OsEvent = OsEvent;
    type OsEvents = Vec<Self::OsEvent>;

    fn new() -> io::Result<Self> {
        // TODO: need to set close on execute on the file descriptor, to stop
        // any forked processes from inheriting a clone of the file descriptor.
        let ret = unsafe { ffi::epoll_create(1) };

        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(Selector {
            epfd: unsafe { OwnedFd::from_raw_fd(ret) },
        })
    }

    fn register(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        // create a new event (dropped at end of this method)
        let mut event = OsEvent {
            events: interest_to_epoll(interests),
            epoll_data: token.0,
        };

        // only use the `add` flag
        let op = ops::EPOLL_CTL_ADD;

        let res = unsafe { ffi::epoll_ctl(self.epfd.as_raw_fd(), op, fd, &mut event) };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    fn reregister(
        &self,
        fd: std::os::unix::prelude::RawFd,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        let mut event = OsEvent {
            events: interest_to_epoll(interests),
            epoll_data: token.0,
        };

        // only use the `add` flag
        let op = ops::EPOLL_CTL_MOD;

        let res = unsafe { ffi::epoll_ctl(self.epfd.as_raw_fd(), op, fd, &mut event) };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    fn deregister(&self, fd: RawFd) -> io::Result<()> {
        // create a new event (dropped at end of this method)
        let op = ops::EPOLL_CTL_DEL;

        let res = unsafe { ffi::epoll_ctl(self.epfd.as_raw_fd(), op, fd, std::ptr::null_mut()) };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    fn poll(
        &self,
        events: &mut Self::OsEvents,
        timeout: Option<std::time::Duration>,
    ) -> io::Result<usize> {
        /// A timeout of -1 means block indefinitely
        /// WARNING: below can truncate on sub-millisecond timeouts
        let timeout = timeout
            .map(|duration| duration.as_millis() as i32)
            .unwrap_or(-1);

        events.clear();

        let ret = unsafe {
            ffi::epoll_wait(
                self.epfd.as_raw_fd(),
                events.as_mut_ptr(),
                events.capacity() as i32,
                timeout,
            )
        };

        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }

        unsafe { events.set_len(ret as usize) };

        Ok(ret as usize)
    }
}

fn interest_to_epoll(interests: Interest) -> i32 {
    // set event to edge-triggered mode by default
    let mut events: i32 = events::EPOLLET;

    if interests.is_readable() {
        events |= events::EPOLLIN | events::EPOLLRDHUP;
    }

    if interests.is_writable() {
        events |= events::EPOLLOUT;
    }

    events
}
