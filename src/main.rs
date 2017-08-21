mod host;
mod message;

use host::*;
use message::*;

fn main() {
    let mut host_a = StreamingHost::new('a');
    let mut host_b = StreamingHost::new('b');
    host_a.bind("127.0.0.1:9001".to_owned());
    host_b.bind("127.0.0.1:9002".to_owned());
    host_a.connect('b', "127.0.0.1:9002".to_owned());
    host_b.connect('a', "127.0.0.1:9001".to_owned());
    host_a.send('b', "Hello, world!");

    for message in host_b {
        println!("{:?}", message);
    }
}
