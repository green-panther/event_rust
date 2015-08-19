extern crate event_rust;
use event_rust::EventLoop;

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    event_loop.run().unwrap();
}