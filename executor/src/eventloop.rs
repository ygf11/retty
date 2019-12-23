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

struct EventLoop {
    servers: HashMap<Token, TcpListener>,
    channels: HashMap<Token, TcpStream>,
    poll: Poll,
    events: Events,
    thread: Builder
}

impl EventLoop {
    fn new() -> EventLoop {
        let poll = match Poll::new() {
            Ok(p) => p,
            Err(e) => panic!("create mio poll failed!"),
        };


        EventLoop {
            poll,
            servers: HashMap::new(),
            channels: HashMap::new(),
            events: Events::with_capacity(128),
            thread: Builder::new(),
        }
    }

    pub fn register(&mut self){

    }

    /// private method
    fn deregister(&mut self){

    }

    /// thread run loop
    fn runLoop(&mut self){

    }
}
