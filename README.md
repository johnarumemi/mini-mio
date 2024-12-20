# Description

Continuation on the miniature project I had started for building my own minimal [mio][1].
This mini private project can be found in the [rust-async/mini-mio][2] private repo.

> [!NOTE] 
> `libc` is not always used, and ffi interfaces are defined within this crate for epoll
> and kqueue directly.


# Running Examples

epoll examples:
- delayserver
- stdinmonitoring


kqueue examples:
- filemonitoring (kqueue)
- timers (kqueue)

---

### delayserver

This makes use of the delayserver found in [rust-async-utils][3]. Hence, that
must be up and running first for this basic example to work correctly.

##### Requirements
- [delayserver][3] from private repo.
- linux OS: uses epoll only

##### Usage

```bash
cargo run --example delayserver
```

---


### stdinmonitoring

Here we are adding standard input's file descriptor (fd0) to epoll's interest list.

##### Requirements
- linux OS: uses epoll only


##### Usage

```bash
cargo run --example stdinmonitoring
```

##### Triggering Events

We trigger events via writing to the proccesses stdin. This can be achieved via
following commands:

1. Get the process id

Using either the below command,
```bash
ps aux | grep "stdinmonitoring"
```

or via reading out the process_id that is logged to stdout when example is first started
up.

2. Write directly into the file descriptor

```bash
echo "<some text input here>" >> /proc/<PID>/fd/0`
```

Where file description zero is typically the standard input.

---

### filemonitoring

This opens a file with the O_EVTONLY flag and adds and event with a system filter that
watches for various types of events to occur to a file descriptor. A separate file
descriptor, with it's own open file description entry, is used in a spawned thread to
write out to the file. The kernel queue is then polled again to retrieve the event
notification on the first file descriptor.

##### Requirements
- BSD / OSX
- kqueue


##### Usage

```bash
cargo run --example filemonitoring
```

##### File mode bitflags

When using the `open` syscall, to create a file if it doesn't exist (O_CREAT), some
additional mode bits must also be passed in as a bitmask to set the permissions on the
temporary file. If these are not set, you could end up with permission errors when
attempting to write to the file after initially creating it.

- [`chmod` man pages on OSX][4]: the permission bits are defined here and not in
  `open`'s man page.

---

### timers

Uses kqueue with timer based system filters.

##### Requirements
- BSD / OSX
- kqueue

##### Usage

```bash
cargo run --example timers
```

---

[1]: https://github.com/tokio-rs/mio "Mio Documentation"
[2]: https://github.com/johnarumemi/rust-async/tree/main/mini-mio "Private rust-async mini-mio"
[3]: https://github.com/johnarumemi/rust-async-utils/tree/main/delayserver "Delayserver"
[4]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chmod.2.html#//apple_ref/doc/man/2/chmod "chmod man pages on OSX"

