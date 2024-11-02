use std::ops::{BitAnd, BitOr};

/// use can also view a similar struct in the `libc` crate on ios/darwin
/// Does not need to be packed. It is 32 bytes anyway, with no padding.
#[derive(Debug)]
#[repr(C)]
pub struct Kevent {
    pub ident: usize,
    /// Identifies the kernel filter used to process this event.
    pub filter: i16, // aliased to i16
    /// Actions to perform on the event.
    pub flags: u16, // aliased to u16
    /// Filter flag.
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
#[allow(non_camel_case_types)]
#[repr(i16)]
pub enum EventFilter {
    // returns whenever there is data available to read.
    EVFILT_READ = -1,
    // returns whenever it is possible to write to the file descriptor.
    EVFILT_WRITE = -2,
    EVFILT_AIO = -3,
    EVFILT_VNODE = -4,
    EVFILT_PROC = -5,
    EVFILT_SIGNAL = -6,

    /// Establishes an interval timer identified by ident where
    /// data specifies the timeout period (in milliseconds).
    ///
    /// `fflags` can include one of the following flags to specify a different unit:
    /// - NOTE_SECONDS   data is in seconds
    /// - NOTE_USECONDS  data is in microseconds
    /// - NOTE_NSECONDS  data is in nanoseconds
    EVFILT_TIMER = -7,
    EVFILT_MACHPORT = -8,
    EVFILT_FS = -9,
    EVFILT_USER = -10,
    EVFILT_VM = -12,
}

#[allow(non_camel_case_types)]
pub enum EventFlag {
    ///  Adds the event to the kqueue. Also automatically enables (EV_ENABLE)
    EV_ADD,
    // Remove the event from the kqueue
    EV_DELETE,
    // Permit kevent() to return the event	if it is triggered
    EV_ENABLE,
    // Disable  the  event	 so  kevent() will not return it
    EV_DISABLE,
    // Deliver one event, then disable the event
    EV_ONESHOT,
    // After the event is retrieved by the	user, its state	is reset.
    EV_CLEAR,
    EV_RECEIPT,
    EV_DISPATCH,
    EV_FLAG0,
    EV_POLL,
    EV_FLAG1,
    EV_OOBAND,
    EV_ERROR,
    EV_EOF,
    EV_SYSFLAGS,
}

pub trait ToValue {
    type Output;

    fn value(&self) -> Self::Output;
}

impl ToValue for EventFlag {
    type Output = u16;
    fn value(&self) -> Self::Output {
        use EventFlag::*;

        match self {
            EV_ADD => 0x1,
            EV_DELETE => 0x2,
            EV_ENABLE => 0x4,
            EV_DISABLE => 0x8,
            EV_ONESHOT => 0x10,
            EV_CLEAR => 0x20,
            EV_RECEIPT => 0x40,
            EV_DISPATCH => 0x80,
            EV_FLAG0 => 0x1000,
            EV_POLL => 0x1000,
            EV_FLAG1 => 0x2000,
            EV_OOBAND => 0x2000,
            EV_ERROR => 0x4000,
            EV_EOF => 0x8000,
            EV_SYSFLAGS => 0xf000,
        }
    }
}

impl From<EventFlag> for u16 {
    fn from(flag: EventFlag) -> Self {
        flag.value()
    }
}

impl<T> BitOr<T> for EventFlag
where
    T: Into<<Self as ToValue>::Output>,
{
    type Output = <Self as ToValue>::Output;

    fn bitor(self, rhs: T) -> Self::Output {
        let rhs = Into::<<Self as ToValue>::Output>::into(rhs);
        self.value() | rhs
    }
}

impl BitOr<EventFlag> for u16 {
    type Output = u16;

    fn bitor(self, rhs: EventFlag) -> Self::Output {
        rhs | self
    }
}

impl<T> BitAnd<T> for EventFlag
where
    T: Into<<Self as ToValue>::Output>,
{
    type Output = <Self as ToValue>::Output;

    fn bitand(self, rhs: T) -> Self::Output {
        let rhs = Into::<<Self as ToValue>::Output>::into(rhs);
        self.value() & rhs
    }
}

#[allow(non_camel_case_types)]
pub enum FilterFlags {
    NOTE_TRIGGER,
    NOTE_FFNOP,
    NOTE_FFAND,
    NOTE_FFOR,
    NOTE_FFCOPY,
    NOTE_FFCTRLMASK,
    NOTE_FFLAGSMASK,
    NOTE_LOWAT,
    // The `unlink(2)` system call was called on the file referenced by the descriptor.
    NOTE_DELETE,
    // A write occurred on the file descriptor
    NOTE_WRITE,
    NOTE_EXTEND,
    /// The file referenced by the descriptor had its attributes changed.
    NOTE_ATTRIB,
    NOTE_LINK,
    NOTE_RENAME,
    NOTE_REVOKE,
    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "visionos",
        target_os = "watchos"
    ))]
    NOTE_NONE,
    NOTE_EXIT,
    NOTE_FORK,
    NOTE_EXEC,
}

impl ToValue for FilterFlags {
    type Output = u32;
    fn value(&self) -> Self::Output {
        use FilterFlags::*;

        match self {
            NOTE_TRIGGER => 0x01000000,
            NOTE_FFNOP => 0x00000000,
            NOTE_FFAND => 0x40000000,
            NOTE_FFOR => 0x80000000,
            NOTE_FFCOPY => 0xc0000000,
            NOTE_FFCTRLMASK => 0xc0000000,
            NOTE_FFLAGSMASK => 0x00ffffff,
            NOTE_LOWAT => 0x00000001,
            NOTE_DELETE => 0x00000001,
            NOTE_WRITE => 0x00000002,
            NOTE_EXTEND => 0x00000004,
            NOTE_ATTRIB => 0x00000008,
            NOTE_LINK => 0x00000010,
            NOTE_RENAME => 0x00000020,
            NOTE_REVOKE => 0x00000040,
            #[cfg(any(
                target_os = "ios",
                target_os = "macos",
                target_os = "tvos",
                target_os = "visionos",
                target_os = "watchos"
            ))]
            NOTE_NONE => 0x00000080,
            NOTE_EXIT => 0x80000000,
            NOTE_FORK => 0x40000000,
            NOTE_EXEC => 0x20000000,
        }
    }
}

impl From<FilterFlags> for u32 {
    fn from(flag: FilterFlags) -> Self {
        flag.value()
    }
}

impl<T> BitOr<T> for FilterFlags
where
    T: Into<<Self as ToValue>::Output>,
{
    type Output = <Self as ToValue>::Output;

    fn bitor(self, rhs: T) -> Self::Output {
        let rhs = Into::<<Self as ToValue>::Output>::into(rhs);
        self.value() | rhs
    }
}

impl<T> BitAnd<T> for FilterFlags
where
    T: Into<<Self as ToValue>::Output>,
{
    type Output = <Self as ToValue>::Output;

    fn bitand(self, rhs: T) -> Self::Output {
        let rhs = Into::<<Self as ToValue>::Output>::into(rhs);
        self.value() & rhs
    }
}

impl BitOr<FilterFlags> for u32 {
    type Output = u32;

    fn bitor(self, rhs: FilterFlags) -> Self::Output {
        rhs | self
    }
}
