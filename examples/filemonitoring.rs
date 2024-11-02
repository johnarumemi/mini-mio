use std::ffi::c_char;
use std::io::Write;
use std::os::fd::{AsRawFd, FromRawFd};
use std::thread;

use libc::{O_CREAT, O_EVTONLY, O_RDWR, O_TRUNC};
use mini_mio::ffi::kqueue::*;

use stat::*;

fn main() {
    // First create the kqueue instance
    let kqfd = unsafe { kqueue() };

    // errors are returned as -1
    assert!(kqfd >= 0);

    let tmpdir = tempdir::TempDir::new("test_kqueue").unwrap();
    let path_buf = tmpdir.path().join("filemonitoring.txt");
    let fpath = path_buf.to_str().unwrap();

    println!("fpath: {}", fpath);

    let bytes = fpath.as_bytes();
    let ptr: *const c_char = bytes.as_ptr().cast();

    let oflag = O_RDWR | O_CREAT | O_TRUNC;

    // set RWX for owner | group | other
    // otherwise, we will get permission errors
    let st_mode = S_IRWXU | S_IRWXG | S_IRWXO;

    // Returns a file descriptor if >= 0
    // or an error if < 0
    let fd_main = unsafe { libc::open(ptr, oflag, st_mode) };
    if fd_main < 0 {
        println!("oflag: {:032b}", oflag);
        println!("{:?}", std::io::Error::last_os_error());
    }
    assert!(fd_main >= 0);

    let fd_evtonly = unsafe { libc::open(ptr, O_EVTONLY) };
    if fd_evtonly < 0 {
        println!("{:?}", std::io::Error::last_os_error());
    }
    println!("FD for open create: {}", fd_main);
    println!("FD for event only : {}", fd_evtonly);
    assert!(fd_evtonly >= 0);

    let user_data = 42;

    use FilterFlags::*;
    let vnode_events = NOTE_DELETE
        | NOTE_WRITE
        | NOTE_EXTEND
        | NOTE_ATTRIB
        | NOTE_LINK
        | NOTE_RENAME
        | NOTE_REVOKE;

    // create an event
    let event = Kevent {
        ident: fd_evtonly as usize,
        filter: EventFilter::EVFILT_VNODE as i16,
        flags: EventFlag::EV_ADD | EventFlag::EV_ENABLE,
        fflags: vnode_events,
        data: fd_evtonly as isize, // flag data
        udata: user_data as usize, // user data
    };

    let changelist = [event];

    let nchanges = changelist.len() as i32;

    let nevents: i32 = 10;
    let mut eventlist = Vec::with_capacity(nevents as usize);

    // register event (no timeout)
    let timeout = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    let ret = unsafe {
        kevent(
            kqfd,
            changelist.as_ptr(),
            nchanges,
            &mut [] as *mut _,
            0,
            &timeout as *const _,
        )
    };
    assert_eq!(ret, 0, "initial registration of event");

    // spawn a thread to write out to the file
    let handle = thread::spawn(move || {
        let output = "hello world".to_string();

        let mut file = unsafe { std::fs::File::from_raw_fd(fd_main) };
        println!("thread fd: {}", file.as_raw_fd());

        println!("writing file...");
        file.write_all(output.as_bytes()).unwrap();
        let _ = file.flush();
        println!("written file successfully");
    });

    handle.join().expect("expected write thread to complete");

    // poll for event
    let timeout = timespec {
        tv_sec: 0,
        tv_nsec: 10000,
    };

    // returns `0` if it times out
    // else returns number of events
    let ret = unsafe {
        kevent(
            kqfd,
            &[] as *const _,
            0,
            eventlist.as_mut_ptr(),
            nevents,
            &timeout as *const _,
        )
    };
    if ret < 0 {
        println!("{0}, {0:?}", std::io::Error::last_os_error());
    } else {
        // must set length of event_list
        unsafe { eventlist.set_len(ret as usize) };
    }

    println!("Changelist: {:?}", &changelist);
    println!("Eventlist:  {:?}", &eventlist);

    assert_eq!(ret, nchanges, "polling for events");

    let ret = unsafe { close(kqfd) };
    assert_eq!(ret, 0);
}

/// File mode bitflags
///
/// These should be or'ed together to form a bitmask.
///
/// See README.md for link to man pages for below bit flags
mod stat {
    #![allow(unused)]

    pub const S_IRWXU: i32 = 0o0000700; /* RWX mask for owner */
    pub const S_IRUSR: i32 = 0o0000400; /* R for owner */
    pub const S_IWUSR: i32 = 0o0000200; /* W for owner */
    pub const S_IXUSR: i32 = 0o0000100; /* X for owner */

    pub const S_IRWXG: i32 = 0o0000070; /* RWX mask for group */
    pub const S_IRGRP: i32 = 0o0000040; /* R for group */
    pub const S_IWGRP: i32 = 0o0000020; /* W for group */
    pub const S_IXGRP: i32 = 0o0000010; /* X for group */

    pub const S_IRWXO: i32 = 0o0000007; /* RWX mask for other */
    pub const S_IROTH: i32 = 0o0000004; /* R for other */
    pub const S_IWOTH: i32 = 0o0000002; /* W for other */
    pub const S_IXOTH: i32 = 0o0000001; /* X for other */

    pub const S_ISUID: i32 = 0o0004000; /* set user id on execution */
    pub const S_ISGID: i32 = 0o0002000; /* set group id on execution */
    pub const S_ISVTX: i32 = 0o0001000; /* save swapped text even after use */
}
