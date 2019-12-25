extern crate executor;

use std::mem;
use std::sync::mpsc::{Sender, channel};
use std::thread::{Builder, Thread};

struct ServerBootStrap {
    sender: Option<Task>,
}

type Task=Sender<Box<dyn FnOnce() -> ()>>;

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

            let builder = Builder::new().spawn(|| {
                // init logic
            });

            Some(sender)
        });

        self.sender = result;
    }

    fn close(&mut self) {}
}