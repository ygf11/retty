use self::super::handlers::Handler;
use std::ops::Deref;

/// channel pipeline
pub struct PipeLine {
    head: Option<*mut Node>,
    tail: Option<*mut Node>,
}

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

    /// iterate from head
    pub fn iter_from_head(&mut self, msg: Vec<u8>, event_type: EventType) {
        let mut cur = self.get_head();
        let message = Message::new(msg);
        while let Some(node) = cur {
            // TODO add handle()
            // handle(&node, message, &event_type);
            cur = self.get_node(node.next);
        }
    }

    /// iterate from tail
    pub fn iter_from_tail(&mut self) {
        let mut cur = self.get_tail();
        while let Some(node) = cur {
            // TODO add handle()
            cur = self.get_node(node.prev);
        }
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
    }
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
}

pub enum EventType {
    READ,
    WRITE,
    REGISTER,
    DEREGISTER,
}

fn handle<T>(node: &Box<Node>, msg: Message<T>, event_type: &EventType){
    match event_type {
        EventType::READ => node.handler.fire_channel_read(),
        EventType::WRITE => node.handler.fire_channel_write(),
        EventType::REGISTER => node.handler.fire_channel_registered(),
        EventType::DEREGISTER => node.handler.fire_channel_deregsiter(),
    }
}

pub struct Message<T> {
    data: Option<T>,
}

impl<T> Message<T> {
    fn new(data: T) -> Message<T> {
        Message {
            data: Some(data)
        }
    }

    fn data(&mut self) -> Option<T> {
        self.data.take()
    }
}