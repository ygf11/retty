use self::super::handlers::Handler;

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
        for _handler in 0..size {
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
    fn new(data: T, _propagate: bool) -> Message<T> {
        Message {
            data: Some(data),
            propagate: _propagate,
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
/*pub trait Chains {
    type Result;

    fn handle_read_event(&mut self, buffer: Vec<u8>) -> Self::Result;

    fn handle_write_event(&mut self, result: Self::Result) -> Vec<u8>;

    fn write(&mut self, result: Self::Result);
}


pub struct DefaultPipeline<F, S, R> {
    inbound_handler: Option<F>,
    outbound_handler: Option<S>,
    phantom: PhantomData<R>,
    write_buffer: Vec<Vec<u8>>,
}

impl<F, S, R> DefaultPipeline<F, S, R> {
    fn new(f: F, s: S) -> DefaultPipeline<F, S, R> {
        DefaultPipeline{
            inbound_handler: Some(f),
            outbound_handler: Some(s),
            phantom: PhantomData,
            write_buffer: Vec::new(),
        }
    }
}

/// TODO en/de coding different object
impl<F, S, T> Chains for DefaultPipeline<F, S, T>
    where F: Fn(&mut DefaultPipeline<F,S,T>, Vec<u8>) -> T,
          S: Fn(&mut DefaultPipeline<F,S,T>, T) -> Vec<u8> {
    type Result = T;

    /// TODO use enum as T
    fn handle_read_event(&mut self, from: Vec<u8>) -> T {
        let handler = self.inbound_handler.take().expect("inbound handler is None.");
        let result = handler(self, from);
        self.inbound_handler = Some(handler);
        //(&self.inbound_handler)(from)
        result
    }

    fn handle_write_event(&mut self, from: T) -> Vec<u8> {
        let handler = self.outbound_handler.take().expect("inbound handler is None.");
        let result = handler(self, from);
        self.outbound_handler = Some(handler);
        //(&self.inbound_handler)(from)
        result
    }

    fn write(&mut self, from: T) {
        let data = self.handle_write_event(from);
        self.write_buffer.push(data);
    }
}
*/


pub trait NewPipeline {

    fn handle_read_event(&mut self, buffer: Vec<u8>);

    fn write(&mut self, buffer:Vec<u8>);

    fn clone(&self) -> Box<dyn NewPipeline + Send>;
}


