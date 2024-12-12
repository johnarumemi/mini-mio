//! Interface to kqueue syscalls.
//!
//! Should not make the syscalls directly, but proxy to `ffi` module instead.
//! Holds the kqueue file descriptor, rather than the Registry holding this.

use std::io;
use std::mem::MaybeUninit;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};

use crate::interests::Interest;
use crate::interfaces::{Event, SysSelector, Token};

// types used for interfacing with kqueue syscalls
use super::events::{fflags, filters, flags, timespec, Kevent};
// kqueue syscalls
use super::ffi;

struct Selector {
    // wrapper around an i32
    // closes the file descriptor when dropped, no need for `close` syscall
    kq: OwnedFd,
}

impl SysSelector for Selector {
    fn new() -> io::Result<Self> {
        // need to set close on execute on the file descriptor, to stop
        // any forked processes from inheriting a clone of the file descriptor.
        let ret = unsafe { ffi::kqueue() };

        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(Selector {
            kq: unsafe { OwnedFd::from_raw_fd(ret) },
        })
    }

    /// A new event needs to be created for each filter being used.
    /// Currently supported filters are for reading and writing only, hence
    /// multiple events might need to be created
    fn register(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        // First thing is to convert out standard `Event` into the `kevent` struct

        let flags = flags::EV_ADD;

        // TODO: move to using preallocated arrays. However, MaybeUninit will not drop T
        // when it is dropped.
        // let mut changelist: [MaybeUninit<Kevent>; 1] = unsafe { MaybeUninit::uninit_array() };

        let mut changelist: Vec<Kevent> = Vec::new();
        let mut nchanges: i32 = 0;

        if interests.is_readable() {
            let kevent = Kevent {
                ident: fd as usize,
                filter: filters::EVFILT_READ,
                flags,
                fflags: 0,
                data: 0,
                udata: token.0,
            };

            changelist.push(kevent);
            // changelist[nchanges as usize] = MaybeUninit::new(kevent);
            nchanges += 1;
        }

        if interests.is_writable() {
            let kevent = Kevent {
                ident: fd as usize,
                filter: filters::EVFILT_WRITE,
                flags,
                fflags: 0,
                data: 0,
                udata: token.0,
            };

            changelist.push(kevent);
            // changelist[nchanges as usize] = MaybeUninit::new(kevent);
            nchanges += 1;
        }

        // Now we can call the `kevent` syscall
        let ret = unsafe {
            ffi::kevent(
                self.kq.as_raw_fd(),
                changelist.as_ptr(),
                nchanges,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
            )
        };

        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }

    fn reregister(
        &self,
        fd: std::os::unix::prelude::RawFd,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        todo!()
    }

    fn deregister(&self, fd: RawFd) -> io::Result<()> {
        let flags: u16 = flags::EV_DELETE | flags::EV_RECEIPT;

        // Need to deregister this file descriptor from 2 filters
        let kevent_read = Kevent {
            ident: fd as usize,
            filter: filters::EVFILT_READ,
            flags,
            fflags: 0,
            data: 0,
            udata: 0,
        };

        let kevent_write = Kevent {
            ident: fd as usize,
            filter: filters::EVFILT_WRITE,
            flags,
            fflags: 0,
            data: 0,
            udata: 0,
        };
        let changelist = [kevent_read, kevent_write];
        let nchanges = changelist.len() as i32;

        // Now we can call the `kevent` syscall
        let ret = unsafe {
            ffi::kevent(
                self.kq.as_raw_fd(),
                changelist.as_ptr(),
                nchanges,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
            )
        };

        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }

    fn poll(
        &self,
        events: &mut [Event],
        timeout: Option<std::time::Duration>,
    ) -> io::Result<usize> {
        todo!()
    }
}
