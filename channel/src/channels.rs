
pub trait Channel{
    fn get(&self);

    fn read(&self);

    fn write(&self);
}