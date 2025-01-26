//! Interface to kqueue syscalls.
//!
//! Should not make the syscalls directly, but proxy to `ffi` module instead.
//! Holds the kqueue file descriptor, rather than the Registry holding this.

use std::io;
use std::mem::MaybeUninit;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};

use crate::interests::Interest;
use crate::interfaces::{SysSelector, Token};

// types used for interfacing with kqueue syscalls
use crate::sys::constants::kqueue::{fflags, filters, flags};
use crate::sys::events::{OsEvent, OsEvents};

// kqueue syscalls
use super::ffi::{self, timespec};

pub struct Selector {
    /// OwnedFd is a wrapper around an i32.
    /// It closes the file descriptor when dropped: no need for `close` syscall.
    kq: OwnedFd,
}

impl SysSelector for Selector {
    type OsEvent = OsEvent;
    type OsEvents = Vec<Self::OsEvent>;

    fn new() -> io::Result<Self> {
        // TODO: need to set close on execute on the file descriptor, to stop
        // any forked processes from inheriting a clone of the file descriptor.
        let ret = unsafe { ffi::kqueue() };

        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(Selector {
            kq: unsafe { OwnedFd::from_raw_fd(ret) },
        })
    }

    fn register(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        // NOTE: A new event needs to be created for each filter being used.
        // Currently supported filters are for reading and writing only, hence
        // multiple events might need to be created

        // It is important to set EV_CLEAR or kqueue will not reset the event after it has been
        // triggered. i.e. by default is behaves in a level triggered mode.
        let flags = flags::EV_CLEAR | flags::EV_RECEIPT | flags::EV_ADD;

        // TODO: move to using preallocated arrays. However, MaybeUninit will not drop T
        // when it is dropped.
        // let mut changelist: [MaybeUninit<OsEvent>; 1] = unsafe { MaybeUninit::uninit_array() };

        let mut changelist: Vec<OsEvent> = Vec::new();
        let mut nchanges: i32 = 0;

        // convert out standard `Event` into the `kevent` struct
        if interests.is_readable() {
            let kevent = OsEvent {
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
            let kevent = OsEvent {
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

        // Below is to enable ease of generating quick tests in this
        // initial implementation phase.
        #[cfg(all(test, target_os = "macos"))]
        {
            if interests.is_timer() {
                let kevent = OsEvent {
                    ident: fd as usize,
                    filter: filters::EVFILT_TIMER,
                    flags: flags | flags::EV_ENABLE,
                    fflags: unsafe { std::mem::zeroed::<u32>() },
                    data: token.0 as isize, // flag data
                    udata: token.0,
                };

                changelist.push(kevent);
                // changelist[nchanges as usize] = MaybeUninit::new(kevent);
                nchanges += 1;
            }
            println!("Changelist: {:?}", changelist);
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
        // I don't believe there are different filters or flags that need
        // to be used when reregistering an event.
        self.register(fd, token, interests)
    }

    fn deregister(&self, fd: RawFd) -> io::Result<()> {
        let flags: u16 = flags::EV_DELETE | flags::EV_RECEIPT;

        // Need to deregister this file descriptor from 2 filters
        let kevent_read = OsEvent {
            ident: fd as usize,
            filter: filters::EVFILT_READ,
            flags,
            fflags: 0,
            data: 0,
            udata: 0,
        };

        let kevent_write = OsEvent {
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
        events: &mut Self::OsEvents,
        timeout: Option<std::time::Duration>,
    ) -> io::Result<usize> {
        let timeout: Option<timespec> = timeout.map(Into::into);

        let timeout = timeout
            .as_ref()
            .map(|s| s as *const _)
            .unwrap_or(std::ptr::null());

        events.clear();

        let ret = unsafe {
            ffi::kevent(
                self.kq.as_raw_fd(),
                std::ptr::null(),
                0,
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn basic() {
        let selector = Selector::new().unwrap();

        let interest = Interest::TIMER;

        // Token is used to set udata and also set the timers wake up time via fflags.
        selector.register(1, Token(10), interest).unwrap();

        let mut eventlist = Vec::with_capacity(10);

        let timeout = Some(std::time::Duration::from_secs(5));
        let ret = selector.poll(&mut eventlist, timeout).unwrap();

        println!("eventlist: {:?}", eventlist);
        println!("syscall return value: {:?}", ret);
    }
}
