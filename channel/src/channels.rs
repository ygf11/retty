use std::net::{TcpStream, TcpListener};

/// channel trait
pub trait Channel{
    fn get(&self);

    fn bind(&mut self);

    fn connect(&mut self);

    fn read(&self);

    fn write(&self);
}

pub struct SocketChannel {
    channel: TcpStream,
    write_buf: Vec<u8>,
}

impl SocketChannel{
    pub fn new(channel:TcpStream) -> SocketChannel{
        SocketChannel{
            channel,
            write_buf:Vec::new(),
        }
    }
}

impl Channel for SocketChannel{
    fn get(&self){

    }

    fn bind(&mut self){

    }

    fn connect(&mut self){

    }

    fn read(&self){

    }

    fn write(&self){

    }
}

pub struct ServerChannel{
    channel: TcpListener,
}

impl ServerChannel {
    pub fn new(channel:TcpStream) -> SocketChannel{
        SocketChannel{
            channel,
            write_buf:Vec::new(),
        }
    }
}


impl Channel for ServerChannel{
    fn get(&self){

    }

    fn bind(&mut self){

    }

    fn connect(&mut self){

    }

    fn read(&self){

    }

    fn write(&self){

    }
}
