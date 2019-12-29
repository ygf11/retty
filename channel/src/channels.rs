use mio::net::{TcpStream, TcpListener};
use self::super::pipeline::PipeLine;
use self::super::handlers::Handler;
use std::net::SocketAddr;
use std::borrow::BorrowMut;
use mio::{Poll, Token, Ready, PollOpt};

/// channel trait
pub trait Channel {
    fn get(&self);

    fn bind(&mut self);

    fn connect(&mut self);

    /// event loop invoke this read method
    fn read(&self);

    /// callback
    fn write(&self);

    /// register interested
    fn register(&self, poll: Poll, token: Token);
}

pub struct SocketChannel {
    channel: TcpStream,
    write_buf: Vec<u8>,
    pipeline: PipeLine,
}

impl SocketChannel {
    pub fn new(channel: TcpStream, handlers: Vec<Box<dyn Handler + Send>>) -> SocketChannel {
        let mut pipeline = PipeLine::new();
        pipeline.add_all(handlers);

        SocketChannel {
            channel,
            pipeline,
            write_buf: Vec::new(),
        }
    }
}

impl Channel for SocketChannel {
    fn get(&self) {
        // 1. read from tcp stream
        // 2. fire event in handler-chain
        // 3. write into tcp stream
        //
        //
    }

    fn bind(&mut self) {
        // unsupport
    }

    fn connect(&mut self) {}

    fn read(&self) {}

    fn write(&self) {}

    fn register(&self, poll: Poll, token: Token) {
        poll.register(&self.channel, token,
                      Ready::readable() | Ready::writable(), PollOpt::edge());
    }
}

pub struct ServerChannel {
    channel: TcpListener,
    pipeline: PipeLine,
}

impl ServerChannel {
    pub fn new(address: SocketAddr,
               handlers: Vec<Box<dyn Handler + Send>>)
               -> Result<ServerChannel, &'static str> {
        let socket = TcpListener::bind(&address).
            map_err(|err| "bind failed.")?;

        let mut pipeline = PipeLine::new();
        pipeline.add_all(handlers);

        let result = ServerChannel {
            pipeline,
            channel: socket,
        };

        Ok(result)
    }
}


impl Channel for ServerChannel {
    fn get(&self) {}

    fn bind(&mut self) {}

    fn connect(&mut self) {
        // unsupport
    }

    fn read(&self) {}

    fn write(&self) {}

    fn register(&self, poll: Poll, token: Token) {}
}
