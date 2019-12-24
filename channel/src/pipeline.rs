use self::super::handlers::Handler;

/// channel pipeline
pub struct PipeLine {
    head: Option<*mut Node>,
    tail: Option<*mut Node>,
}

impl PipeLine {
    fn new() -> PipeLine {
        PipeLine {
            head: None,
            tail: None,
        }
    }

    /// add last
    pub fn add_last(&mut self, handler: Box<dyn Handler>) {
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


