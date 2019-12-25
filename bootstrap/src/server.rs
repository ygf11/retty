extern crate executor;

use std::mem;
use std::sync::mpsc::{Sender, channel};
use std::thread::{Builder, Thread};
use executor::eventloop::EventLoop;
use self::executor::eventloop::{Message, Operation};


struct ServerBootStrap {
    sender: Option<Sender<Message>>,
}


impl ServerBootStrap {
    fn new() -> ServerBootStrap {
        ServerBootStrap {
            sender: None,
        }
    }

    /// send msg to thread
    fn bind(&mut self, address: &str) {
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

        // register
        self.sender.as_ref().map(move|sender| {
            let message = Operation::Bind(String::from(address));
            sender.send(message);
        });
    }

    fn close(&mut self) {}
}