# Event - Async IO similar to libevent

Event is a lightweight IO library for Rust with a focus on adding as
little overhead as possible over the OS abstractions.

[![Build Status](https://api.travis-ci.org/tickbh/event_rust.svg?branch=master)](https://travis-ci.org/tickbh/event_rust)

**Getting started guide**
Currently a work in progress:

## Usage

To use `event_rust`, first add this to your `Cargo.toml`:

```toml
[dependencies]
event_rust = "0.1.1"
```

Then, add this to your crate root:

```rust
extern crate event_rust;
```

Add empty event just do
```rust
extern crate event_rust;
use event_rust::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.run();
}
```

Add simple timer event just do
```rust
extern crate event_rust;
use event_rust::{EventLoop, EventEntry, EventFlags};
use std::ptr;

fn time_callback(ev : *mut EventLoop, fd : u64, _ : EventFlags, data : *mut ()) -> i32 {
    println!("fd is {:?}", fd);
    //return 0 status ok other will delete the timer
    0
}

pub fn main() {
    let mut event_loop : EventLoop = EventLoop::new().unwrap();
    event_loop.add_timer(EventEntry::new_timer(100, false, Some(time_callback), Some( ptr::null_mut() )));
    event_loop.add_timer(EventEntry::new_timer(200, true, Some(time_callback), Some( ptr::null_mut() )));
    event_loop.run().unwrap();
}
```

## Features

* Event loop backed by epoll, windows by select.
* Non-blocking TCP sockets
* High performance timer system

## Platforms

Currently, event_rust only supports Linux and Windows. The goal is to support
all platforms that support Rust and the readiness IO model.
