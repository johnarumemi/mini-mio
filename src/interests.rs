//! Abstractions used for registering types of interests on a socket.
#![allow(dead_code)]

use std::num::NonZeroU8; // Also see std::ptr::NonNull;
use std::ops::BitOr;

/// Interest is a bit flag that represents the operations that a socket is interested in.
///
/// Non-zero bit flags that must be unique from each other.
/// They are publicly available as constant values defined in the Interest type
const READABLE: u8 = 1; // 0b00000001
const WRITABLE: u8 = 1 << 1; // 0b00000010
#[cfg(all(test, target_os = "macos"))]
const TIMER: u8 = 1 << 2; // 0b00000100

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Interest(NonZeroU8);

impl Interest {
    // Named constant value of type `Interest`
    // This is not associated with a memory location on the type, but
    // are inlined into the context they are used.

    pub const READABLE: Interest = Interest(unsafe { NonZeroU8::new_unchecked(READABLE) });
    pub const WRITABLE: Interest = Interest(unsafe { NonZeroU8::new_unchecked(WRITABLE) });

    #[cfg(all(test, target_os = "macos"))]
    pub const TIMER: Interest = Interest(unsafe { NonZeroU8::new_unchecked(TIMER) });

    /// Add an interest via a bitwise or
    /// returns a new owned `Interest`
    pub fn add_interest(self, other: Interest) -> Interest {
        Interest(self.0 | other.0)
    }

    /// Remove interests via a bitwise &!
    ///
    /// a value of zero represents None
    /// use ! on rhs to set all bits to keep to true
    /// use & to keep all bits from rhs in lfs
    /// lhs       = 101101
    /// rhs       = 001000
    /// !rhs      = 110111
    /// lhs &!rhs = 100101 , 4th bit, from right, is now 0
    pub fn remove_interest(self, other: Interest) -> Option<Interest> {
        NonZeroU8::new(self.0.get() & !other.0.get()).map(Interest)
    }

    pub fn is_readable(&self) -> bool {
        (self.0.get() & Self::READABLE.0.get()) != 0
    }

    pub fn is_writable(&self) -> bool {
        (self.0.get() & Self::WRITABLE.0.get()) != 0
    }

    #[cfg(all(test, target_os = "macos"))]
    pub fn is_timer(&self) -> bool {
        (self.0.get() & Self::TIMER.0.get()) != 0
    }
}

impl std::fmt::Debug for Interest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut previous = false;

        if self.is_readable() {
            write!(f, "READABLE")?;
            previous = true
        }

        if self.is_writable() {
            if previous {
                write!(f, " | ")?
            }
            write!(f, "WRITABLE")?;
        }

        #[cfg(all(test, target_os = "macos"))]
        if self.is_timer() {
            if previous {
                write!(f, " | ")?
            }
            write!(f, "TIMER")?;
        }

        Ok(())
    }
}

// Implementing the From trait for converting from Interest to u8
impl From<Interest> for u8 {
    fn from(value: Interest) -> Self {
        value.0.get()
    }
}

// Implementing the From trait for converting from NonZeroU8 to Interest
impl From<NonZeroU8> for Interest {
    fn from(value: NonZeroU8) -> Self {
        Interest(value)
    }
}

impl BitOr for Interest {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.add_interest(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_interest() {
        let interest = Interest::READABLE;

        assert!(interest.is_readable());
        assert!(!interest.is_writable());

        let interest = interest.add_interest(Interest::WRITABLE);

        assert!(interest.is_readable());
        assert!(interest.is_writable());

        if cfg!(target_os = "macos") {
            assert!(!interest.is_timer());
            let interest = interest.add_interest(Interest::TIMER);
            assert!(interest.is_timer());
        }
    }
}
