#![allow(unused)]

use std::io;

use crate::interests::Interest;
use crate::interfaces::Token;

use std::os::fd::RawFd;
use std::time::Duration;

use super::SysEvent;

pub trait SysSelector
where
    Self: Sized,
{
    type OsEvent: SysEvent;
    type OsEvents: AsMut<Vec<Self::OsEvent>>;

    /// Create a new instance of the OSes event queue and store event queue file descriptor
    fn new() -> io::Result<Self>;

    /// Register interest in events on a sources file descriptor.
    ///
    /// The `Interest` is all that we require to know how to create the relevant event queue
    /// abstraction to be passed in the syscall.
    fn register(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()>;

    /// Modify interest in events on a sources file descriptor
    ///
    /// The `Interest` is all that is required to know how to create the relevant event queue
    /// abstraction to be passed in the syscall.
    ///
    /// The `RawFd` is just an alias to c_int, which is an i32 on unix / OSX.
    fn reregister(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()>;

    /// Poll for events on file descriptors
    fn poll(&self, events: &mut Self::OsEvents, timeout: Option<Duration>) -> io::Result<usize>;

    /// Stop monitoring for events on file descriptor
    fn deregister(&self, fd: RawFd) -> io::Result<()>;
}
