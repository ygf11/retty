extern crate executor;

use std::mem;
use std::sync::mpsc::{Sender, channel};
use std::thread::{Builder, Thread};
use executor::eventloop::{EventLoop, Task};


struct ServerBootStrap {
    sender: Option<Sender<Task>>,
}


impl ServerBootStrap {
    fn new() -> ServerBootStrap {
        ServerBootStrap {
            sender: None,
        }
    }

    /// send msg to thread
    fn bind(&mut self) {
        let result = self.sender.take().or_else(|| {
            let (sender, receiver) = channel();
            let sender_clone = sender.clone();

            let builder = Builder::new().spawn(move || {
                // init logic
                let mut event_loop = EventLoop::new(sender_clone, receiver);
                event_loop.run_loop();
            });

            Some(sender)
        });

        self.sender = result;
    }

    fn close(&mut self) {}
}