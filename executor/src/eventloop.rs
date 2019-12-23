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

struct EventLoop {
    sender: Sender<Box<dyn Task>>,
    receiver: Receiver<Box<dyn Task>>,
    poll: Poll,
    events: Events,
    thread: Builder,
    channels: HashMap<Token, Box<dyn Channel>>,
}

impl EventLoop {
    fn new() -> EventLoop {
        let poll = match Poll::new() {
            Ok(p) => p,
            Err(e) => panic!("create mio poll failed!"),
        };

        let (sender, receiver) = channel();

        EventLoop {
            poll,
            sender,
            receiver,
            channels: HashMap::new(),
            thread: Builder::new(),
            events: Events::with_capacity(128),
        }
    }

    pub fn register(&mut self, channel: impl Channel) {}

    /// private method
    fn deregister(&mut self) {}

    /// thread run loop
    fn run_loop(&mut self) {
        let mut poll = &mut self.poll;
        let mut events = &mut self.events;
        let mut channels = &mut self.channels;
        loop {
            poll.poll(events, Some(Duration::from_millis(100)));

            for event in events.iter(){
                match event.token() {
                    reader => if event.readiness().is_readable(){

                    }

                    writer => if event.readiness().is_writable(){

                    }
                }
            }
        }
    }

    pub fn producer(&self) -> Sender<Box<dyn Task>>{
        self.sender.clone()
    }

    pub fn execute(&mut self, task: Box<dyn Task>) {
        self.sender.clone().send(task);
    }
}

trait Task{
    fn run(&mut self);
}