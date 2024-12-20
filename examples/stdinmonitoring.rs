//! This example demonstrates how to monitor
//! standard input for read events using epoll.

#![allow(dead_code, unused)]

use std::io::{Read, Result, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::process;

use mini_mio::ffi::epoll::{self as ffi, Event};
use mini_mio::poll::{self, Poll};

fn main() -> Result<()> {
    let process_id = process::id();
    let docstring = format!(
        r#"
=======================================
Running with process id: {}
=======================================
"#,
        process_id
    );

    println!("{}", docstring);

    let a = i32::MAX;
    // Create a new event queue
    let mut poll = Poll::new()?;

    // Options for how we want the file to be opened
    let mut options = std::fs::OpenOptions::new();

    options.create(true);
    options.write(true);
    options.read(true);

    let mut stdin = std::io::stdin();

    let num_events = 5;

    let max_events = 10;
    let mut events = Vec::with_capacity(max_events);

    // Register interest in being notified when file is ready to read
    poll.registry().register(
        &stdin,                      // source
        0,                           // token
        ffi::EPOLLIN | ffi::EPOLLET, // bitmask for read + edge-triggered
    )?;

    // We will attempt to read from the file a few times
    for _ in 0..num_events {
        // Register interest in being notified when file is ready to read

        println!("Blocking on poll...");
        poll.poll(&mut events, None)?;
        println!("Woke up from poll...");

        // let mut buffer = vec![0u8; 4096]; // 4KB buffer
        let mut buffer = String::new();

        let n = stdin.read_line(&mut buffer)?;

        print!("{buffer}");
    }

    Ok(())
}
