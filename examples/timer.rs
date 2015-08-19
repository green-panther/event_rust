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