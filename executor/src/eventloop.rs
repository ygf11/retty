extern crate channel;
extern crate mio;

use mio::Poll;
use mio::Events;
use mio::net::TcpStream;
use mio::Token;
use mio::net::TcpListener;
use channel::channels::Channel;
use std::collections::HashMap;
use std::thread::Thread;
use std::thread::Builder;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::future::Future;
use std::alloc::handle_alloc_error;

struct EventLoop<'a> {
    poll: Poll,
    events: Events,
    thread: Option<&'a Thread>,
    thread_builder: Builder,
    sender: Sender<Box<dyn FnOnce() -> () + Send>>,
    receiver: Receiver<Box<dyn FnOnce() -> () + Send>>,
    channels: HashMap<Token, Box<dyn Channel>>,
}

impl<'a> EventLoop<'a> {
    fn new() -> EventLoop<'a> {
        let poll = match Poll::new() {
            Ok(p) => p,
            Err(e) => panic!("create mio poll failed!"),
        };

        let (sender, receiver) = channel();

        EventLoop {
            poll,
            sender,
            receiver,
            thread: None,
            channels: HashMap::new(),
            thread_builder: Builder::new(),
            events: Events::with_capacity(128),
        }
    }

    pub fn register<F>(&mut self)
                       -> Result<Sender<Box<dyn FnOnce() -> () + Send>>, &'static str> {
        // 1. thread none => start thread
        // 2. thread not none => add register task

        //let thread = self.thread.or_else(|| {
        //    let result = self.thread_builder.spawn(|| {
        //        self.run_loop();
        //    });

        //    result.or_else(|_| panic!("start thread failed."));

        //   Some(result.unwrap().thread())
        //});

        //self.thread.or_else(|| thread);

        Ok(self.sender.clone())
    }

    /// private method
    fn deregister(&mut self) {}

    /// thread run loop
    fn run_loop(&mut self) {
        let mut poll = &mut self.poll;
        let mut events = &mut self.events;
        let mut channels = &mut self.channels;
        loop {
            poll.poll(events, Some(Duration::from_millis(100)));

            for event in events.iter() {
                match event.token() {
                    reader => if event.readiness().is_readable() {}

                    writer => if event.readiness().is_writable() {}
                }

                // TODO handle tasks
            }
        }
    }

    pub fn producer<F>(&self) -> Sender<Box<dyn FnOnce() -> () + Send>> {
        self.sender.clone()
    }

    pub fn execute<F>(&mut self, task: Box<dyn FnOnce() -> () + Send>) {
        self.sender.clone().send(task);
    }
}

trait Task {
    fn run(&mut self);

    fn after(&mut self);
}