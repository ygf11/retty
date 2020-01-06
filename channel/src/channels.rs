use mio::net::{TcpStream, TcpListener};
use self::super::pipeline::PipeLine;
use self::super::handlers::Handler;
use std::net::SocketAddr;
use std::borrow::BorrowMut;
use mio::{Poll, Token, Ready, PollOpt};
use std::rc::Rc;
use crate::pipeline::NewPipeline;
use std::sync::mpsc::Sender;
use std::error::Error;

/// channel trait
pub trait Channel {
    fn get(&self);

    /// event loop invoke this read method
    fn read(&mut self);

    /// callback
    fn write(&self);

    fn accept(&mut self) -> Result<TcpStream, &'static str>;

    /// register interested
    fn register(&self, poll: &Poll, token: Token);

    fn is_server(&self) -> bool;
}

pub struct SocketChannel {
    channel: TcpStream,
    write_buf: Vec<u8>,
    pipeline: Box<dyn NewPipeline + Send>,
}

impl SocketChannel {
    pub fn new(channel: TcpStream, handler: Box<dyn NewPipeline + Send>) -> SocketChannel {
        SocketChannel {
            channel,
            pipeline: handler,
            write_buf: Vec::new(),
        }
    }
}

impl Channel for SocketChannel {
    fn get(&self) {}

    fn read(&mut self) {
        // 1. read from socket
        // 2. pipeline.handle_channel_read()
        // 3. pipeline.handle_channel_write()
        // 4. channel.write()
    }

    fn write(&self) {}

    fn accept(&mut self) -> Result<TcpStream, &'static str> {
        panic!("unsupport operation for Socket channel.")
    }

    fn register(&self, poll: &Poll, token: Token) {
        poll.register(&self.channel, token,
                      Ready::readable() | Ready::writable(), PollOpt::edge());
    }

    fn is_server(&self) -> bool{
        false
    }
}

pub struct ServerChannel {
    channel: TcpListener,
    pipeline: Box<dyn NewPipeline + Send>,
}

impl ServerChannel {
    pub fn new(address: SocketAddr,
               handler: Box<dyn NewPipeline + Send>)
               -> Result<ServerChannel, &'static str> {
        let socket = TcpListener::bind(&address).
            map_err(|err| "bind failed.")?;

        let result = ServerChannel {
            pipeline: handler,
            channel: socket,
        };

        Ok(result)
    }
}


impl Channel for ServerChannel {
    fn get(&self) {}

    /// accept connection
    fn read(&mut self) {
        panic!("unsupport operation for serverChannel.")
    }

    fn write(&self) {}

    fn accept(&mut self) -> Result<TcpStream, &'static str> {
        let channel = &mut self.channel;
        match channel.accept() {
            Ok((tcpStream, addr)) => Ok(tcpStream),
            Err(e) => Err("accept connection error."),
        }
    }

    fn register(&self, poll: &Poll, token: Token) {
        poll.register(&self.channel, token,
                      Ready::readable() | Ready::writable(), PollOpt::edge());
    }

    fn is_server(&self) -> bool{
        true
    }
}
