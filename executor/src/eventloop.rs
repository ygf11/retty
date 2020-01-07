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
use self::channel::channels::{SocketChannel, ServerChannel};
use crate::token::Tokens;
use self::channel::handlers::Handler;
use std::net::SocketAddr;
use self::channel::pipeline::NewPipeline;

pub struct EventLoop {
    poll: Poll,
    tokens: Tokens,
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
            tokens: Tokens::new(),
            receiver: Some(receiver),
            task_queue: Some(Vec::new()),
            thread: thread::current(),
            channels: HashMap::new(),
            events: Events::with_capacity(128),
        }
    }

    pub fn register(&mut self, channel: Box<dyn Channel>) {
        let token = self.next_token();
        self.channels.insert(token, channel);

        // register read/write event
        self.channels.get(&token).map(|channel|
            channel.register(&self.poll, token));
    }

    /// private method
    fn deregister(&mut self) {}

    /// thread run loop
    pub fn run_loop(&mut self) {
        loop {
            let poll = &mut self.poll;
            let events = &mut self.events;
            let channels = &mut self.channels;

            poll.poll(events, Some(Duration::from_millis(100)));

            for event in events.iter() {
                match event.token() {
                    // read and accept
                    reader => if event.readiness().is_readable() {},

                    // write event
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
            // bind
            Operation::Bind(address, handler) => {
                let server = ServerChannel::new(address, handler);

                match server {
                    Ok(channel) => self.register(Box::new(channel)),
                    Err(e) => println!("{:?}", e),
                }
            }

            // connect
            Operation::Connect(address, handler) => {
                let client = TcpStream::connect(&address);

                match client {
                    Ok(channel) =>
                        self.register(Box::new(SocketChannel::new(channel, handler))),
                    Err(e) => println!("{:?}", e)
                }
            }
        }
    }

    fn run_local_task(&mut self, task: &LocalTask) {
        // run local task
        // let channel = task.channel;
    }

    fn next_token(&mut self) -> Token {
        // TODO UNIQUE
        self.tokens.next()
    }

    fn handle_read_or_accept_event(&mut self, token: Token) {
        let mut value = self.channels.remove(&token);

        if let Some( channel) = value.as_mut() {
            match channel.is_server() {
                true => self.handle_accept_event(channel),
                false => self.handle_read_event(channel),
            }
        }

        if let Some(channel) = value {
            self.channels.insert(token, channel);
        }
    }

    fn handle_write_event(&self, token: Token) {}

    fn handle_read_event(&mut self, channel: &mut Box<dyn Channel>) {}

    fn handle_accept_event(&mut self, channel: &mut Box<dyn Channel>) {}
}

pub enum Operation {
    Bind(SocketAddr, Box<dyn NewPipeline + Send>),
    Connect(SocketAddr, Box<dyn  NewPipeline + Send>),
}

pub struct LocalTask {
    channel: Option<SocketChannel>,
}


pub type Message = Operation;