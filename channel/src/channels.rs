use mio::net::{TcpStream, TcpListener};
use self::super::pipeline::PipeLine;
use self::super::handlers::Handler;
use std::net::SocketAddr;

/// channel trait
pub trait Channel {
    fn get(&self);

    fn bind(&mut self);

    fn connect(&mut self);

    /// event loop invoke this read method
    fn read(&self);

    /// callback
    fn write(&self);
}

pub struct SocketChannel {
    channel: TcpStream,
    write_buf: Vec<u8>,
    pipeline: PipeLine,
}

impl SocketChannel {
    pub fn new(channel: TcpStream) -> SocketChannel {
        SocketChannel {
            channel,
            write_buf: Vec::new(),
            pipeline: PipeLine::new(),
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
}

pub struct ServerChannel {
    channel: TcpListener,
    pipeline: PipeLine,
}

impl ServerChannel {
    pub fn new(address: &SocketAddr, handlers:Vec<Box<dyn Handler>>) -> Result<ServerChannel, &'static str> {
        let socket = TcpListener::bind(address)
            .or_else(|err| Err("bind failed."))?;

        let result = ServerChannel {
            channel: socket,
            pipeline: PipeLine::new(),
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
}
