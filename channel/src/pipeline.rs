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
}


struct Node {
    prev: Option<*mut Node>,
    next: Option<*mut Node>,
    handler: Box<dyn Handler>,
}

impl Node{
    fn new(handler: Box<dyn Handler>) -> Node{
        Node{
            handler,
            prev:None,
            next:None,
        }
    }

    fn into_element(self:Box<Self>) -> Box<dyn Handler>{
        self.handler
    }
}


