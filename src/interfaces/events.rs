#![allow(unused)]
use std::{
    iter::Iterator,
    ops::{Deref, DerefMut},
};

use super::Event;

/// Wrapper around the OsEvents type.
pub struct Events {
    inner: crate::sys::OsEvents,
}

impl From<Events> for crate::sys::OsEvents {
    fn from(events: Events) -> Self {
        events.inner
    }
}

impl Events {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: crate::sys::OsEvents::with_capacity(capacity),
        }
    }
}

impl Deref for Events {
    type Target = crate::sys::OsEvents;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Events {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// ################ IntoIter ################
pub struct IntoIter(Events);

/// Defines how `Events` can be converted into an iterator
///
/// This is a consuming iterator.
impl IntoIterator for Events {
    type Item = Event;
    type IntoIter = IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl Iterator for IntoIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.inner.pop().map(Event::new)
    }
}

// ################ Iter ################

/// Hold a reference to the next Event we want to yield
///
/// This should also hold a mutable reference the `Events` collection
/// to ensure it is not mutated while we are iterating over it.
///
/// # Safety
///
/// This was trickey to implement. By trying to iterator of Vec<OsEvents> via
/// mutable reference, we ended up having `&OsEvent` being returned. Now, what
/// we really want to do is simply have the `Event` type proxy function calls to
/// the `OsEvent`. There is no actual structrual data changes introduced via use
/// of this type. So if the memory layout of `Event` = `OsEvent`, we cast a
/// &OsEvent to a &Event. This will require use of the `#[repr(transparent)]`
/// attribute on the Event type. I have used this in the past for other
/// purposes, but usually with single valued tuple structs. Lets experiment to
/// see if it works with single valued struct fields.
pub struct Iter<'a> {
    /// inner iterator that yields references to an OsEvent
    inner: std::slice::Iter<'a, crate::sys::OsEvent>,
}

impl Events {
    /// Returns an iterator over mutable references.
    ///
    /// Implemented via wrapping an inner iterator that yields OsEvents,
    /// and returning this as an iterator that yields Events.
    fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.inner.iter(),
        }
    }
}

impl<'a> IntoIterator for &'a Events {
    type Item = &'a Event;
    type IntoIter = Iter<'a>;

    // note thate `self` = &'a Events
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Event;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Event::ref_from_sys_event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Events {
        fn push(&mut self, event: crate::sys::OsEvent) {
            self.inner.push(event)
        }
    }

    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    #[test]
    fn test_events_is_send() {
        is_send::<Events>();
    }

    #[test]
    fn test_events_is_sync() {
        is_sync::<Events>();
    }

    #[test]
    fn into_iter() {
        let os_event = crate::sys::OsEvent::default();

        let mut events = Events::with_capacity(1);

        events.push(os_event);

        let mut iter = events.into_iter();

        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter() {
        let os_event = crate::sys::OsEvent::default();

        let mut events = Events::with_capacity(1);

        events.push(os_event);

        let mut iter = events.iter();

        assert!(matches!(iter.next(), Some(&_)));
        assert!(iter.next().is_none());
    }
}
