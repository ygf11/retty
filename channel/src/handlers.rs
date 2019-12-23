pub trait Handler{
    /// in bound
    fn handle_in_bound(&self);

    /// out bound
    fn handle_out_bound(&self);

    fn handle(&self);
}



