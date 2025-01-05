#![allow(unused)]

/// This system call is used to add, modify, or remove entries in the interest list of the
/// epoll(7) instance referred to by the file descriptor epfd.  It requests that the operation
/// op be performed for the target file descriptor, fd.
/// taken from : /usr/include/aarch64-linux-gnu/sys/epoll.h
pub(crate) mod ops {
    /// Add an entry to the interest list of the epoll file
    /// descriptor, epfd.  The entry includes the file descriptor,
    /// fd, a reference to the corresponding open file description
    /// (see epoll(7) and open(2)), and the settings specified in
    /// event.
    pub const EPOLL_CTL_ADD: i32 = 1;

    /// Remove (deregister) the target file descriptor fd from the
    /// interest list.  The event argument is ignored and can be
    /// NULL (but see BUGS below).
    pub const EPOLL_CTL_DEL: i32 = 2;

    /// Change the settings associated with fd in the interest
    /// list to the new settings specified in event.
    pub const EPOLL_CTL_MOD: i32 = 3;
}

/// The events member of the epoll_event structure is a bit mask
/// composed by ORing together zero or more event types, returned by
/// epoll_wait(2), and input flags, which affect its behaviour, but
/// aren't returned.  The available event types are:
pub(crate) mod events {

    /// The associated file is available for read(2) operations.
    pub const EPOLLIN: i32 = 0x001;

    /// The associated file is available for write(2) operations.
    pub const EPOLLOUT: i32 = 0x004;

    /// There is an exceptional condition on the file descriptor.
    /// Possibilities include:
    /// - There is out-of-band data on a TCP socket (see tcp(7)).
    /// - A pseudoterminal master in packet mode has seen a state
    ///   change on the slave (see ioctl_tty(2)).
    /// - A cgroup.events file has been modified (see
    ///   cgroups(7)).
    pub const EPOLLPRI: i32 = 0x002;
    pub const EPOLLRDNORM: i32 = 0x040;
    pub const EPOLLRDBAND: i32 = 0x080;
    pub const EPOLLWRNORM: i32 = 0x100;
    pub const EPOLLWRBAND: i32 = 0x200;
    pub const EPOLLMSG: i32 = 0x400;

    /// Error condition happened on the associated file descriptor.  This event
    /// is also reported for the write end of a pipe when the read end has been
    /// closed. epoll_wait(2) will always report for this event; it is not
    /// necessary to set it in events when calling epoll_ctl()
    pub const EPOLLERR: i32 = 0x008;

    /// Hang up happened on the associated file descriptor.
    /// epoll_wait(2) will always wait for this event; it is not
    /// necessary to set it in events when calling epoll_ctl().
    /// Note that when reading from a channel such as a pipe or a
    /// stream socket, this event merely indicates that the peer
    /// closed its end of the channel.  Subsequent reads from the
    /// channel will return 0 (end of file) only after all
    /// outstanding data in the channel has been consumed.
    pub const EPOLLHUP: i32 = 0x010;

    /// Stream socket peer closed connection, or shut down writing
    /// half of connection.  (This flag is especially useful for
    /// writing simple code to detect peer shutdown when using
    /// edge-triggered monitoring.)
    pub const EPOLLRDHUP: i32 = 0x2000;

    // The available input flags are:
    mod inputs {

        /// Sets an exclusive wakeup mode for the epoll file descriptor that is
        /// being attached to the target file descriptor, fd.  When a wakeup event
        /// occurs and multiple epoll file descriptors are attached to the same
        /// target open file description entry, using EPOLLEXCLUSIVE, one or more of
        /// the epoll file descriptors will receive an event with epoll_wait(2).
        /// The default in this scenario (when EPOLLEXCLUSIVE is not set) is for all
        /// epoll file descriptors to receive an event. EPOLLEXCLUSIVE is thus
        /// useful for avoiding thundering herd problems in certain scenarios.
        pub const EPOLLEXCLUSIVE: i32 = 1 << 28;

        /// If EPOLLONESHOT and EPOLLET are clear and the process has
        /// the CAP_BLOCK_SUSPEND capability, ensure that the system
        /// does not enter "suspend" or "hibernate" while this event
        /// is pending or being processed.  The event is considered as
        /// being "processed" from the time when it is returned by a
        /// call to epoll_wait(2) until the next call to epoll_wait(2)
        /// on the same epoll(7) file descriptor, the closure of that
        /// file descriptor, the removal of the event file descriptor
        /// with EPOLL_CTL_DEL, or the clearing of EPOLLWAKEUP for the
        /// event file descriptor with EPOLL_CTL_MOD.  See also BUGS.
        pub const EPOLLWAKEUP: i32 = 1 << 29;

        /// Requests one-shot notification for the associated file descriptor.
        pub const EPOLLONESHOT: i32 = 1 << 30;

        /// Requests edge-triggered notification for the associated
        /// file descriptor.  The default behavior for epoll is level-
        /// triggered.
        pub const EPOLLET: i32 = 1 << 31;
    }

    pub use inputs::*;
}
