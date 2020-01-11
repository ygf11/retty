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
use std::io;
use std::io::Read;

/// channel trait
pub trait Channel {
    fn get(&self);

    fn child_handler(&self) -> Box<dyn NewPipeline + Send>;

    /// event loop invoke this read method
    fn read(&mut self) -> Vec<u8>;

    /// callback
    fn write(&self);

    fn accept(&mut self) -> Result<TcpStream, &'static str>;

    /// register interested
    fn register(&self, poll: &Poll, token: Token);

    fn deregister(&self, poll: &Poll);

    fn is_server(&self) -> bool;

    fn fire_channel_read(&mut self, buffer: Vec<u8>);
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

    fn child_handler(&self) -> Box<dyn NewPipeline + Send> {
        panic!("unsupport operation for socket channel.");
    }

    fn read(&mut self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut array = buffer.as_mut_slice();
        let mut read: usize = 0;
        loop {
            let channel = &mut self.channel;
            let result = channel.read(&mut array[read..]);
            match result {
                Ok(0) => {
                    // close
                }
                Ok(n) => {
                    read += n;
                }

                Err(ref err) if would_block(err) => break,

                Err(ref err)  if interrupted(err) => continue,

                // todo return err
                Err(err) => break,
                // other error
            }
        }

        buffer
    }

    fn write(&self) {}

    fn accept(&mut self) -> Result<TcpStream, &'static str> {
        panic!("unsupport operation for Socket channel.")
    }

    fn register(&self, poll: &Poll, token: Token) {
        poll.register(&self.channel, token,
                      Ready::readable() | Ready::writable(), PollOpt::edge());
    }

    fn deregister(&self, poll: &Poll) {
        poll.deregister(&self.channel);
    }

    fn is_server(&self) -> bool {
        false
    }

    fn fire_channel_read(&mut self, buffer: Vec<u8>) {
        // 1. read from socket
        // 2. pipeline.handle_channel_read()
        // 3. pipeline.handle_channel_write()
        // 4. channel.write()
        self.pipeline.handle_read_event(buffer);
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

    fn child_handler(&self) -> Box<dyn NewPipeline + Send> {
        self.pipeline.clone()
    }

    /// accept connection
    fn read(&mut self) -> Vec<u8> {
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

    fn deregister(&self, poll: &Poll) {
        poll.deregister(&self.channel);
    }

    fn is_server(&self) -> bool {
        true
    }

    fn fire_channel_read(&mut self, buffer: Vec<u8>) {
        panic!("unsupport operation for serverChannel.")
    }
}

fn would_block(err: &std::io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &std::io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}