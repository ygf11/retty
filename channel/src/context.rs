use self::super::channels::Channel;

/// inbound context
/// read event
struct InboundContext{
    read_buffer:Vec<u8>,
}


/// outbound context
/// write event
struct OutboundContext{
    write_buffer:Vec<u8>,
}