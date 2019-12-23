extern crate channel;
extern crate mio;

use mio::Poll;
use mio::Events;
use channel::channel::Channel;

struct EventLoop {
    channel: Channel,
    poll: Poll,
    events: Events,
}