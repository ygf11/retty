use crate::pipeline::Message;

pub trait Handler {
    /*
    fn fire_channel_read<T, R>(&self, data: Message<T>) -> Message<R>;

    fn fire_channel_write<T, R>(&self, data: Message<T>) -> Message<R>;

    fn fire_channel_registered<T>(&self) -> Message<T>;

    fn fire_channel_deregsiter<T>(&self) -> Message<T>;

    fn fire_next_read(&self);

    fn fire_next_write(&self);

    fn fire_next_register(&self);

    fn fire_next_deregister(&self);

    fn need_fire_next(&self) -> bool;

    fn reset(&self);
    */
}
