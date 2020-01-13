extern crate executor;
extern crate channel;

use std::sync::mpsc::{Sender, channel};
use std::thread::{Builder, Thread};
use executor::eventloop::EventLoop;
use self::executor::eventloop::{Message, Operation};
use self::channel::handlers::Handler;
use std::net::SocketAddr;
use self::channel::pipeline::NewPipeline;


struct ServerBootStrap {
    sender: Option<Sender<Message>>,
}

impl ServerBootStrap {
    fn new(pipeline: Box<dyn NewPipeline + Send>) -> ServerBootStrap {
        ServerBootStrap {
            sender: None,
        }
    }

    /// send msg to thread
    fn bind(&mut self,
            addr: SocketAddr,
            pipeline:Box<dyn NewPipeline + Send>) {
        self.create_eventloop();

        // register
        self.sender.as_ref().map(move |sender| {
            let message = Operation::Bind(addr, pipeline);
            let result = sender.send(message);
            if let Err(_err) = result{
                // log
            }
        });
    }

    fn connect(&mut self,
               addr:SocketAddr, pipeline:Box<dyn NewPipeline + Send>){
        self.create_eventloop();

        self.sender.as_ref().map(move |sender| {
            let message = Operation::Connect(addr, pipeline);
            let result = sender.send(message);
            if let Err(_err) = result{
                // log
            }
        });
    }

    fn create_eventloop(&mut self){
        let result = self.sender.take().or_else(|| {
            let (sender, receiver) = channel();
            let sender_clone = sender.clone();

            let builder = Builder::new().spawn(move || {
                // init logic
                let mut event_loop = EventLoop::new(sender_clone, receiver);
                event_loop.run_loop();
            });

            if builder.is_err(){
                panic!("err");
            }

            Some(sender)
        });

        self.sender = result;
    }

    fn close(&mut self) {}
}