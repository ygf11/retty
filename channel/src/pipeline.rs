use self::super::handlers::Handler;
use std::ops::Deref;
use std::panic::resume_unwind;
use std::rc::Weak;
use std::marker::PhantomData;

/// channel pipeline
pub struct PipeLine {
    head: Option<*mut Node>,
    tail: Option<*mut Node>,
}

/// TODO replace linked list with handler array
impl PipeLine {
    pub fn new() -> PipeLine {
        PipeLine {
            head: None,
            tail: None,
        }
    }

    /// add last
    pub fn add_last(&mut self, handler: Box<dyn Handler + Send>) {
        unsafe {
            let mut node = Box::new(Node::new(handler));
            node.next = None;
            node.prev = self.tail;

            let raw_ptr = Some(Box::into_raw(node));

            match self.head {
                None => self.tail = raw_ptr,
                Some(head) => (*head).prev = raw_ptr,
            }

            self.tail = raw_ptr;
        }
    }

    pub fn add_all(&mut self, handlers: Vec<Box<dyn Handler + Send>>) {
        let mut handlers = handlers;
        let size = handlers.len();
        for handler in 0..size {
            self.add_last(handlers.remove(0))
        }
    }

    /*
    /// iterate from head
    pub fn fire_channel_read(&mut self, msg: Vec<u8>) {
        let mut cur = self.get_head();
        let message = Message::new(msg, true);
        let event_type = EventType::READ;
        cur.map(|node| handle(&node, message, &event_type));
    }

    /// iterate from tail
    pub fn fire_channel_write(&mut self, msg: Vec<u8>, event_type: EventType) {
        let mut cur = self.get_tail();
        let message = Message::new(msg, true);

        let event_type = EventType::WRITE;
        cur.map(|node| handle(&node, message, &event_type));
    }

    /// iterate from head
    pub fn fire_channel_registered(&mut self) {
        let mut cur = self.get_head();
        let message = Message::<u8>::empty(true);
        let event_type = EventType::REGISTER;
        cur.map(|node| handle(&node, message, &event_type));
    }

    /// iterate from head
    pub fn fire_channel_deregistered(&mut self) {
        let mut cur = self.get_head();
        let message = Message::<u8>::empty(true);
        let event_type = EventType::DEREGISTER;
        cur.map(|node| handle(&node, message, &event_type));
    }


    fn get_head(&mut self) -> Option<Box<Node>> {
        unsafe {
            self.head.map(|node| {
                Box::from_raw(node)
            })
        }
    }

    fn get_tail(&mut self) -> Option<Box<Node>> {
        unsafe {
            self.tail.map(|node| {
                Box::from_raw(node)
            })
        }
    }

    fn get_node(&self, node: Option<*mut Node>) -> Option<Box<Node>> {
        unsafe {
            node.map(|node| {
                Box::from_raw(node)
            })
        }
    }*/
}

struct Node {
    prev: Option<*mut Node>,
    next: Option<*mut Node>,
    handler: Box<dyn Handler>,
}

impl Node {
    fn new(handler: Box<dyn Handler>) -> Node {
        Node {
            handler,
            prev: None,
            next: None,
        }
    }

    fn into_element(self: Box<Self>) -> Box<dyn Handler> {
        self.handler
    }

    fn get_next(&self) -> Option<Box<Node>> {
        unsafe {
            self.next.map(|node| {
                Box::from_raw(node)
            })
        }
    }

    fn get_prev(&self) -> Option<Box<Node>> {
        unsafe {
            self.prev.map(|node| {
                Box::from_raw(node)
            })
        }
    }

    /*
    fn fire_channel_read<T>(&self, message: Message<T>) {
        let result = self.handler.fire_channel_read(message);

        if result.propagate {
            let next = self.get_next();
            next.map(|node| {
                node.fire_channel_read(result)
            });
        }

        self.handler.reset();
    }

    fn fire_channel_write<T>(&self, message: Message<T>) {
        let result = self.handler.fire_channel_write(message);

        if result.propagate {
            let next = self.get_prev();
            next.map(|node| {
                node.fire_channel_write(message)
            });
        }

        self.handler.reset();
    }

    fn fire_channel_registered(&self) {
        let result = self.handler.fire_channel_registered();

        if result.propagate {
            let next = self.get_next();
            next.map(|node| {
                node.fire_channel_registered()
            });
        }

        self.handler.reset();
    }

    fn fire_channel_deregistered(&self) {
        let result = self.handler.fire_channel_deregsiter();

        if result.propagate {
            let next = self.get_next();
            next.map(|node| {
                node.fire_channel_deregistered()
            });
        }

        self.handler.reset();
    }
    */
}

pub enum EventType {
    READ,
    WRITE,
    REGISTER,
    DEREGISTER,
}
/*
fn handle<T>(node: &Box<Node>, msg: Message<T>, event_type: &EventType) {
    match event_type {
        EventType::READ => node.fire_channel_read(msg),
        EventType::WRITE => node.fire_channel_write(msg),
        EventType::REGISTER => node.handler.fire_channel_registered(),
        EventType::DEREGISTER => node.handler.fire_channel_deregsiter(),
    }
}*/

pub struct Message<T> {
    propagate: bool,
    data: Option<T>,
}

impl<T> Message<T> {
    fn new(data: T, propagate: bool) -> Message<T> {
        Message {
            data: Some(data),
            propagate: false,
        }
    }

    fn empty(propagate: bool) -> Message<T> {
        Message {
            data: None,
            propagate,
        }
    }

    fn data(&mut self) -> Option<T> {
        self.data.take()
    }
}

pub struct ChannelResult<T> {
    propagate: bool,
    data: Message<T>,
}

impl<T> ChannelResult<T> {
    fn new(propagate: bool, data: Message<T>) -> ChannelResult<T> {
        ChannelResult {
            propagate,
            data,
        }
    }
}


/// other try of chain
trait Chains {
    type Result;

    fn handle_read_event(&mut self, buffer: Vec<u8>) -> Self::Result;

    fn handle_write_event(&mut self, result: Self::Result) -> Vec<u8>;
}


struct NewPipeline<F, S, R> {
    inbound_handler: F,
    outbound_handler: S,
    phantom: PhantomData<R>,
}

impl<F, S, R> NewPipeline<F, S, R> {
    fn new(f: F, s: S) -> NewPipeline<F, S, R> {
        NewPipeline {
            inbound_handler: f,
            outbound_handler: s,
            phantom: PhantomData,
        }
    }
}

/// TODO en/de coding different object
impl<F, S, T> Chains for NewPipeline<F, S, T>
    where F: Fn(Vec<u8>) -> T,
          S: Fn(T) -> Vec<u8> {
    type Result = T;

    /// TODO use enum as T
    fn handle_read_event(&mut self, from: Vec<u8>) -> T {
        (&self.inbound_handler)(from)
    }

    fn handle_write_event(&mut self, from: T) -> Vec<u8> {
        (&self.outbound_handler)(from)
    }
}
