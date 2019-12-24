use self::super::handlers::Handler;

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

    /// iterate from head
    pub fn iter_from_head(&mut self) {
        let mut cur = self.get_head();
        while let Some(node) = cur {
            // TODO add handle()
            cur = self.get_node(node.next);
        }
    }

    /// iterate from tail
    pub fn iter_from_tail(&mut self){
        let mut cur = self.get_tail();
        while let Some(node) = cur{
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

    fn get_node(&self, node:Option<*mut Node>) -> Option<Box<Node>>{
        unsafe{
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