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
use std::mem;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use self::channel::channels::SocketChannel;

pub struct EventLoop {
    poll: Poll,
    events: Events,
    thread: Thread,
    sender: Sender<Message>,
    receiver: Option<Receiver<Message>>,
    task_queue: Option<Vec<LocalTask>>,
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
            receiver: Some(receiver),
            task_queue: Some(Vec::new()),
            thread: thread::current(),
            channels: HashMap::new(),
            events: Events::with_capacity(128),
        }
    }

    pub fn register(&mut self, channel:impl Channel) {

    }

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
        self.run_remote_tasks();
        self.run_local_tasks();
    }

    fn run_remote_tasks(&mut self) {
        // remote
        let receiver = self.receiver.take()
            .expect("none receiver in eventloop.");

        loop {
            match receiver.try_recv() {
                Ok(task) => self.run_remote_task(task),
                Err(_) => break,
            }
        }

        self.receiver = Some(receiver);
    }

    fn run_local_tasks(&mut self) {
        // local
        let queue = self.task_queue.take()
            .expect("none task_queue in eventloop.");

        for task in queue.iter() {
            self.run_local_task(task);
        }

        self.task_queue = Some(queue);
    }

    fn run_remote_task(&mut self, operation: Message) {
        match operation {
            Operation::Bind(address) => println!(""),
            Operation::Connect(address) => println!(""),
        }
    }

    fn run_local_task(&mut self, task: &LocalTask) {
        // run local task
    }
}

pub enum Operation {
    Bind(String),
    Connect(String),
}

pub struct LocalTask {
    channel: SocketChannel,
}


pub type Message = Operation;