pub trait Handler{

    fn fire_channel_read(&self);

    fn fire_channel_write(&self);

    fn fire_channel_registered(&self);

    fn fire_channel_deregsiter(&self);
}



