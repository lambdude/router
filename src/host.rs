use std::net::{TcpListener, TcpStream, SocketAddrV4};
use std::collections::HashMap;
use std::io::{Write, Read};

use message::*;

// let listener = TcpListener::bind("127.0.0.1:80").unwrap();
//
// fn handle_client(stream: TcpStream) {
//     // ...
// }
//
// // accept connections and process them serially
// for stream in listener.incoming() {
//     match stream {
//         Ok(stream) => {
//             handle_client(stream);
//         }
//         Err(e) => { /* connection failed */ }
//     }
// }

enum HostError {
    BindError,
    ConnectError,
    SendError,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct HostId(pub char);

impl<'a> Into<HostId> for &'a char {
    fn into(self) -> HostId {
        HostId(*self)
    }
}

pub trait Host {
    fn new<I: Into<HostId>>(id: I) -> Self;
    fn bind(&mut self, socket: String) -> Result<(), HostError>;
    fn connect<I: Into<HostId>>(&mut self,id: I, socket: String) -> Result<(), HostError>;
    fn send<I: Into<HostId>>(&self, destination: I, message: &str) -> Result<(), HostError>;
}

pub struct StreamingHost {
    id: HostId,
    stream: Option<TcpStream>,
    listener: Option<TcpListener>,
    connections: HashMap<HostId, TcpStream>
}

impl From<char> for HostId {
    fn from(character: char) -> Self {
        HostId(character)
    }
}

impl Host for StreamingHost {
    fn new<I: Into<HostId>>(id: I) -> Self {
        StreamingHost {
            id: id.into(),
            listener: None,
            stream: None,
            connections: HashMap::new()
        }
    }

    fn bind(&mut self, socket: String) -> Result<(), HostError> {
        self.listener = Some(TcpListener::bind(socket).map_err(|_| HostError::BindError)?);
        Ok(())
    }

    fn connect<I: Into<HostId>>(&mut self, id: I, socket: String) -> Result<(), HostError> {
        let host_id = id.into();

        if self.connections.contains_key(&host_id) {
            return Err(HostError::ConnectError);
        } else {
            let stream = TcpStream::connect(socket).map_err(|_| HostError::ConnectError)?;
            self.connections.insert(host_id, stream);
            Ok(())
        }
    }

    fn send<I: Into<HostId>>(&self, id: I, string: &str) -> Result<(),HostError> {
        let destination = id.into();
        if let Some(mut stream) = self.connections.get(&destination) {
            let message = StringMessage::new(self.id.clone(), destination.clone(), String::from(string));
            stream.write(&message.payload());
        } else {
            return Err(HostError::SendError);
        }
        Ok(())
    }
}

impl Iterator for StreamingHost {
    type Item = StringMessage;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stream.is_none() {
            if let Some(ref listener) = self.listener {
                self.stream = listener.incoming().next().unwrap().ok();
            }
        }

        if let Some(ref mut stream) = self.stream {
            let mut buffer = [0; 100];
            stream.read(&mut buffer);
            Some(StringMessage::from_bytes(buffer))
        } else {
            None
        }
    }
}
