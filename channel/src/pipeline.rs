use self::super::handlers::Handler;

/// channel pipeline
pub struct PipeLine{
    head: Option<*mut Node>,
    tail: Option<*mut Node>,
}

impl PipeLine {
    fn new() -> PipeLine{
        PipeLine{
            head:None,
            tail:None
        }
    }
}


struct Node{
    handler: Box<dyn Handler>,
    prev: Option<*mut Node>,
    next: Option<*mut Node>,
}





