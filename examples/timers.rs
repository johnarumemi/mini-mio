//! This example demonstrates how to use the
//! kqueue API to create a simple timer based event.

use mini_mio::ffi::kqueue::*;

fn main() {
    // First create the kqueue instance
    let kqfd = unsafe { kqueue() };

    // errors are returned as -1
    assert!(kqfd >= 0);

    let flags = EventFlag::EV_ADD | EventFlag::EV_ENABLE;

    let changelist = vec![event(1, 1000, flags), event(2, 1000, flags)];

    let nchanges = changelist.len() as i32;

    let nevents: i32 = 10;
    let mut eventlist = Vec::with_capacity(nevents as usize);

    let t = timespec {
        tv_sec: 4,
        tv_nsec: 0,
    };

    let ret = unsafe {
        kevent(
            kqfd,
            changelist.as_ptr(),
            nchanges,
            eventlist.as_mut_ptr(),
            nevents,
            &t as *const _,
        )
    };

    println!("Changelist: {:?}", &changelist);
    println!("Eventlist:  {:?}", &eventlist);

    if ret < 0 {
        println!("{0}, {0:?}", std::io::Error::last_os_error());
    }
    assert_eq!(ret, nchanges);

    let ret = unsafe { close(kqfd) };
    assert_eq!(ret, 0);
}

/// Helper method for creating a timer based filter
fn event(id: usize, timer: isize, flags: u16) -> Kevent {
    Kevent {
        ident: id,
        filter: EventFilter::EVFILT_TIMER as i16,
        flags,
        fflags: unsafe { std::mem::zeroed::<u32>() },
        data: timer, // flag data
        udata: 0,
    }
}
