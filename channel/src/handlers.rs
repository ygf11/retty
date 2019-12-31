pub trait Handler {
    fn fire_channel_read<T, R>(&self, msg: T) -> R;

    fn fire_channel_write<T, R>(&self, msg: T) -> R;

    fn fire_channel_registered(&self);

    fn fire_channel_deregsiter(&self);
}



