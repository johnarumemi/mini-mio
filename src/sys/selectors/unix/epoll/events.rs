use std::ops::{BitAnd, BitOr};

#[repr(i32)]
pub enum EpollEvents {
    /// The associated file is available for read(2) operations.
    EPOLLIN = 0x001,
    /// There is an exceptional condition on the file descriptor.
    EPOLLPRI = 0x002,
    /// The associated file is available for write(2) operations.
    EPOLLOUT = 0x004,
    EPOLLRDNORM = 0x040,
    EPOLLRDBAND = 0x080,
    EPOLLWRNORM = 0x100,
    EPOLLWRBAND = 0x200,
    EPOLLMSG = 0x400,
    EPOLLERR = 0x008,
    /// Hang up happened on the associated file descriptor.
    EPOLLHUP = 0x010,
    EPOLLRDHUP = 0x2000,
    /// Sets an exclusive wakeup mode for the epoll file
    /// descriptor that is being attached to the target file
    /// descriptor, fd.  When a wakeup event occurs and multiple
    /// epoll file descriptors are attached to the same target
    /// open file description entry, using EPOLLEXCLUSIVE, one or more of the epoll file
    /// descriptors will receive an event with epoll_wait(2).  The
    /// default in this scenario (when EPOLLEXCLUSIVE is not set)
    /// is for all epoll file descriptors to receive an event.
    /// EPOLLEXCLUSIVE is thus useful for avoiding thundering herd
    /// problems in certain scenarios.
    EPOLLEXCLUSIVE = 1 << 28,
    EPOLLWAKEUP = 1 << 29,
    /// Requests one-shot notification for the associated file descriptor.
    EPOLLONESHOT = 1 << 30,
    /// Requests edge-triggered notification for the associated
    /// file descriptor.  The default behavior for epoll is level-
    /// triggered.
    EPOLLET = 1 << 31,
}

impl EpollEvents {
    /// Check if a flag has been set within the bitmask
    pub fn is_set(self, bitmask: i32) -> bool {
        bitmask & (self as i32) != 0
    }
}
impl BitOr for EpollEvents {
    type Output = i32;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as i32 | rhs as i32
    }
}

impl BitAnd for EpollEvents {
    type Output = i32;
    fn bitand(self, rhs: Self) -> Self::Output {
        self as i32 & rhs as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_operations() {
        use EpollEvents::*;

        let interests = EPOLLIN | EPOLLET;

        assert!(EPOLLIN.is_set(interests));
        assert!(EPOLLET.is_set(interests));

        assert!(!EPOLLEXCLUSIVE.is_set(interests));
    }
}
