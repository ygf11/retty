extern crate channel;
extern crate mio;

use mio::Poll;
use mio::Events;
use mio::net::TcpStream;
use mio::Token;
use mio::net::TcpListener;
use channel::channels::Channel;
use std::collections::HashMap;
use std::thread::{Thread, Builder};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

pub struct EventLoop {
    poll: Poll,
    events: Events,
    thread: Thread,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    channels: HashMap<Token, Box<dyn Channel>>,
}

impl EventLoop {
    pub fn new(sender: Sender<Message>, receiver: Receiver<Message>) -> EventLoop {
        let poll = match Poll::new() {
            Ok(p) => p,
            Err(e) => panic!("create mio poll failed!"),
        };

        EventLoop {
            poll,
            sender,
            receiver,
            thread: thread::current(),
            channels: HashMap::new(),
            events: Events::with_capacity(128),
        }
    }

    pub fn register(&mut self) {}

    /// private method
    fn deregister(&mut self) {}

    /// thread run loop
    pub fn run_loop(&mut self) {
        loop {
            let mut poll = &mut self.poll;
            let mut events = &mut self.events;
            let mut channels = &mut self.channels;

            poll.poll(events, Some(Duration::from_millis(100)));

            for event in events.iter() {
                match event.token() {
                    reader => if event.readiness().is_readable() {}

                    writer => if event.readiness().is_writable() {}
                }
            }

            // TODO handle tasks
            self.run_tasks();


        }
    }

    pub fn producer(&self) -> Sender<Message> {
        self.sender.clone()
    }

    pub fn execute(&mut self, task: Message) {
        self.sender.clone().send(task);
    }

    fn run_tasks(&mut self) {
        let receiver = &mut self.receiver;
        loop {
            let message= receiver.try_recv();


            match message {
                Ok(task) => println!("message"),
                Err(_) => break,
            }
        }
    }
}

pub enum Operation {
    Bind(String),
    Connect(String),
}

pub type Message = Operation;