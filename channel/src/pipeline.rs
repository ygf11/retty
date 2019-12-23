use self::super::handlers::Handler;

/// channel pipeline
pub struct PipeLine{
    handlers: Vec<Box<dyn Handler>>,
}

impl PipeLine {
    pub fn new() -> PipeLine{
        PipeLine{
            handlers:Vec::new()
        }
    }


}
