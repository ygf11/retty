pub trait Handler {
    fn fire_channel_read(&self);

    fn fire_channel_write(&self);

    fn fire_channel_registered(&self);

    fn fire_channel_deregsiter(&self);

    fn fire_next_read(&self);

    fn fire_next_write(&self);

    fn fire_next_register(&self);

    fn fire_next_deregister(&self);

    fn need_fire_next(&self) -> bool;

    fn reset(&self);
}


pub trait Chain {

}
