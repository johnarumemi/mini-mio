# Description

Continuation on the miniature project I had started for building my own minimal [mio][1].
This mini private project can be found in the [rust-async/mini-mio][2] private repo.

> [!NOTE] 
> `libc` is not always used, and ffi interfaces are defined within 
> this crate for epoll and kqueue directly.

[1]: https://github.com/tokio-rs/mio "Mio Documentation"
[2]: https://github.com/johnarumemi/rust-async/tree/main/mini-mio "Private rust-async mini-mio"

# Running Examples

#### delayserver

`cargo run --example delayserver`

This makes use of the delayserver found in [rust-async-utils][3]. Hence, that
must be up an running first for this basic example to work correct.y


_Requirements_

- [delayserver][3]
- linux OS: current implementation uses epoll only, so will not working on any other platform.


<!-- Links -->
[3]: https://github.com/johnarumemi/rust-async-utils/tree/main/delayserver "Delayserver"


#### stdinmonitoring

`cargo run --example stdinmonitoring`

Here we are adding standard input's file descriptor to epoll's interest list.

We trigger events via writing to the proccesses stdin. This can be achieved via following:

1. Get the process id

`ps aux | grep "stdinmonitoring"`

or via reading out the process_id that I have logged out when example is first
started up.

2. `echo "<some text input here>" >> /proc/<PID>/fd/0`

where file description zero is typically the standard input.

Note: if running via vscode devcontainer, ensure you either 
#### filemonitoring

`cargo run --example filemonitoring`

__Requirements__
- BSD / OSX
- kqueue

This opens a file with the O_EVTONLY flag and adds and event with a system filter that watches for
various types of events to occur to a file descriptor. A separate file descriptor, with it's own
open file description entry, is used in a spawned thread to write out to the file. The kernel queue
is then polled again to retrieve the event notification on the first file descriptor.

##### File mode bitflags

When using the `open` syscall, to create a file if it doesn't exist (O_CREAT), some additional mode
bits must also be passed in as a bitmask to set the permissions on the temporary file. If these are
not set, you could end up with permission errors when attempting to write to the file after
initially creating it.

- [`chmod` man pages on OSX][4]: the permission bits are defined here and not in `open`'s man page.

[4]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chmod.2.html#//apple_ref/doc/man/2/chmod

#### timers

`cargo run --example timers`

__Requirements__
- BSD / OSX
- kqueue

Uses kqueue with timer based system filters.

