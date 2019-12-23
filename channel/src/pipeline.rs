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

    pub fn fire_channel_registered(&self){

    }

    pub fn fire_channel_deregsiter(&self){

    }

    pub fn fire_channel_read(&self){

    }

    pub fn fire_channel_write(&self){

    }
}
