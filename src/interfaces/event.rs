//! Event interface for handling an OS specific event.
#![allow(unused)]
use crate::interfaces::Token;
use crate::sys;

use super::sysevent::SysEvent;

/// Review: I could honestly just bypass the whole use of the trait `SysEvent`
/// and the below type alias by just setting `inner: crate::sys::Event`.
/// However, I am using the `SysEvent` to guarantee that all required methods
/// are implemented in OS specific Event and correctly proxied to in `GenericEvent`.
#[allow(unused)]
pub type Event = GenericEvent<sys::OsEvent>;

impl Event {
    pub fn new(inner: sys::OsEvent) -> Self {
        Self { inner }
    }

    /// Convert from &OsEvent to &Event via raw pointer casting.
    ///
    /// Required for implmentation of Iter<'a>
    pub fn ref_from_sys_event(os_event: &sys::OsEvent) -> &Self {
        unsafe { &*(os_event as *const sys::OsEvent as *const Self) }
    }
}

/// Wrapper around OS specific Event types
///
/// It is important this this struct only contains one field, allowing the use of the
/// repr(transparent) attribute. This attribute guarantees that the struct has the same layout as
/// the field it contains. This allows us to safely cast between the raw immutable pointer to an
/// OsEvent to a raw immutable pointer to a GenericEvent instead and allowing access to the
/// GenericEvent methods.
///
/// Review: Is it really safe to clone these if underlying OsEvents might contain
/// fields with pointers? Copying the pointer values might lead to double free.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct GenericEvent<T>
where
    T: SysEvent,
{
    inner: T,
}

/// TODO: This is technically wrong, since the GenericEvent also implements SysEvent. Allowing
/// it to technically wrap itself.
/// The SysEvent now also needs to be public to enable usage of the `token()` method.
impl<T> GenericEvent<T>
where
    T: SysEvent,
{
    pub fn from_sys_event(inner: T) -> Self {
        Self { inner }
    }
}

trait WrapperEvent {}

impl<T> SysEvent for GenericEvent<T>
where
    T: SysEvent,
{
    fn token(&self) -> Token {
        self.inner.token()
    }

    fn is_readable(&self) -> bool {
        self.inner.is_readable()
    }

    fn is_read_closed(&self) -> bool {
        self.inner.is_read_closed()
    }

    fn is_writable(&self) -> bool {
        self.inner.is_writable()
    }

    fn is_write_closed(&self) -> bool {
        self.inner.is_write_closed()
    }

    fn is_error(&self) -> bool {
        self.inner.is_error()
    }
}
