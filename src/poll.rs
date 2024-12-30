#![allow(unused)]

use std::{
    io::{self, Result},
    net::TcpStream,
    os::fd::AsRawFd,
    time::Duration,
};

use crate::interests::Interest;
use crate::interfaces::{Event, Events, SysSelector, Token};
use crate::sys::selectors::Selector;

pub trait Source: AsRawFd {}

impl<T> Source for T where T: AsRawFd {}
/// Represents the event queue itself.
pub struct Poll {
    /// A Registry is specific to an event queue / Poll instance
    registery: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        Ok(Self {
            registery: Registry::new()?,
        })
    }

    /// return reference to the registry that can be used for registering
    /// interest to be notified of new events on a source file descriptor.
    pub fn registry(&self) -> &Registry {
        &self.registery
    }

    /// Blocks / parks the current thread it's called on until an event is ready or timeout occurs.
    pub fn poll(&mut self, events: &mut Events, timeout: Option<Duration>) -> Result<()> {
        // REVIEW: confirm casting of &mut Events -> &mut SysSelector::OsEvents
        self.registery.selector.poll(events, timeout)?;

        Ok(())
    }
}

/// Wraps OS specific selector that manages all syscalls
/// to the OSes event queue abstraction.
pub struct Registry {
    selector: Selector,
}

impl Registry {
    /// Register interest in events on a sources file descriptor.
    ///
    /// - `interests`: The types of events we want to be notified about.
    /// - `token`: user supplied identifier to keep track of the source.
    pub fn register<S: Source>(&self, source: &S, token: Token, interests: Interest) -> Result<()> {
        self.selector.register(source.as_raw_fd(), token, interests)
    }

    fn new() -> Result<Self> {
        Ok(Registry {
            selector: Selector::new()?,
        })
    }
}

// NOTE: `Selector` uses `OwnedFd`, which will close the file descriptor when dropped.
// So we can skip implementing `Drop` trait for now.
//
// impl Drop for Registry {
//     fn drop(&mut self) {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;
}
