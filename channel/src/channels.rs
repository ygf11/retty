use std::net::{TcpStream, TcpListener};

/// channel trait
pub trait Channel{
    fn get(&self);

    fn read(&self);

    fn write(&self);
}

pub struct SocketChannel {
    channel: TcpStream,
}

impl SocketChannel{
    pub fn new(channel:TcpStream) -> SocketChannel{
        SocketChannel{
            channel
        }
    }
}

impl Channel for SocketChannel{
    fn get(&self){

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
            channel
        }
    }
}


impl Channel for ServerChannel{
    fn get(&self){

    }

    fn read(&self){

    }

    fn write(&self){

    }
}
