#![allow(dead_code, unused)]

use mini_mio::interests::Interest;
use mini_mio::interfaces::{Event, Events, SysEvent, SysSelector, Token};
use mini_mio::poll::*;

use std::{
    collections::HashSet,
    io::{self, Read, Result, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

fn main() -> Result<()> {
    // Create a new event queue
    let mut poll = Poll::new()?;

    // max events we want to create for this example program
    let num_events = 2;

    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());

    let socket_addr = format!("{host}:8080");

    // store stream id's that we have handled / gotten a response for
    let mut handled_ids: HashSet<usize> = HashSet::new();

    // Open a connections to a server and send http request.
    // Store tcp streams to be polled for read events later.
    let mut streams = send_requests(poll.registry(), num_events, &socket_addr)?;

    println!("Completed sending all requests and registering streams with epoll\n\n");

    let padding = (0..15).map(|_| "-").collect::<String>();
    let msg = format!("{padding} Starting Event Loop {padding}");
    let boundary = (0..msg.len()).map(|_| "-").collect::<String>();

    println!("\n{boundary}\n{msg}\n{boundary}\n");

    // Now handle read notifications
    let mut handled_events = 0;

    // do below while we haven't got a response from all the requests
    // Note that we are using edge-triggered mode, so we need to drain the buffer completely.
    while handled_events < num_events {
        let mut events = Events::with_capacity(10);

        // poll for events
        poll.poll(&mut events, None)?;

        // reach here when thread is woken up
        if events.is_empty() {
            println!("TIMEOUT OR SPURIOUS WAKEUP EVENT NOTIFICATION");
            continue;
        }

        handled_events += handle_events(&events, &mut streams, &mut handled_ids)?;
    }

    println!("FINISHED PROGRAM");
    Ok(())
}

fn send_requests(
    registry: &Registry,
    num_events: usize,
    socket_addr: &str,
) -> Result<Vec<TcpStream>> {
    let mut streams = vec![];

    // Open a connections to a server and send http request.
    // Store tcp streams to be polled for read events later.
    for i in 0..num_events {
        println!(">>> Starting Request {i} >>>\n");
        // first request has longest timeout, so expect
        // responses to arrive in reverse order.
        let delay = (num_events - i) * 1000;
        println!("Delay: {} ms, for event i = {}", delay, i);
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        println!(
            "Attempting to establish a connection: socket_addr: {}",
            socket_addr
        );
        let mut stream = TcpStream::connect(socket_addr)?;
        println!("Connection established");

        // Disable the Nagle algorithm. This algorithm is enabled by default in Rust
        // implementations, and it can cause a delay in sending packets. It pools together
        // packets and sends them all at once, which can useful for reducing network congestion.
        println!("Disabling Nagle on stream...");
        stream.set_nodelay(true);

        println!("Setting stream to nonblocking mode...");
        // set non-blocking mode
        stream.set_nonblocking(true)?;

        println!("Writing out to stream...");
        // send packet across stream / socket (non-blocking mode is enabled atm)
        stream.write_all(&request)?;

        // sleep for a while to simulate network latency
        // and also ensure requests arrive in order on the server
        thread::sleep(Duration::from_millis(50));

        // register interest in being notified when stream is ready to read
        let token = Token(i);

        let interests = Interest::READABLE;

        println!("Registering stream {i} with event queue...");
        registry.register(&stream, token, interests)?;

        // store stream
        println!("Storing stream...");
        streams.push(stream);

        println!("\n<<< Completed Request {i} <<<\n\n");
    }

    Ok(streams)
}

fn get_req(path: &str) -> Vec<u8> {
    let req = format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    );

    req.into_bytes()
}

fn handle_events(
    events: &Events,
    streams: &mut [TcpStream],
    handled_ids: &mut HashSet<usize>,
) -> Result<usize> {
    let mut handled_events = 0;

    println!("\n------------------------------------\n");
    println!("Handling events...");
    println!("num_events: {}", events.len());
    println!("\n------------------------------------\n");

    for event in events {
        let token = event.token();
        let identifier = token.0;
        println!("Processing event {identifier}: {event:?}");
        println!("\n------------------------------------\n");

        let mut buffer = vec![0u8; 4096]; // 4KB buffer

        let mut i = 0_usize;
        let mut txt = String::new();
        let mut new_response = true;

        loop {
            // use a loop to ensure we drain the buffer.
            // This is important for edge-triggered mode, as if the buffer isn't
            // drained, then it will never reset to notify us of new events.
            match streams[identifier].read(&mut buffer) {
                Ok(0) => {
                    // read 0 bytes - buffer has been drained successfully

                    // `insert` returns false if the value already existed in the set.
                    if !handled_ids.insert(identifier) {
                        println!("Event already handled");
                        break;
                    }

                    handled_events += 1;

                    println!(
                        "\n\nBuffer drained after {i} iteration(s), breaking out of loop...\n"
                    );
                    println!("------------------------------------\n");
                    i = 0;
                    new_response = true;
                    break;
                }
                Ok(n) => {
                    // read in `n` bytes successfully
                    let txt = String::from_utf8_lossy(&buffer[..n]);
                    if new_response {
                        println!("\n--- Response ---");
                        new_response = false;
                    }
                    print!("{txt}");
                    i = i.saturating_add(1);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    println!("\n\nWouldBlock error, breaking out of loop...");
                    break;
                }
                // if the read operation is interrupted (e.g. signal from OS), we can continue
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                    println!("\n\nnRead operation interrupted, continuing...");
                    break;
                }
                Err(e) => return Err(e),
            }
        }
    }

    Ok(handled_events)
}
