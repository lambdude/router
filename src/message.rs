use host::*;

pub trait Message {
    fn source(&self) -> HostId;
    fn destination(&self) -> HostId;
    fn payload(&self) -> [u8; 100];
}

#[derive(Debug)]
pub struct StringMessage {
    source: HostId,
    destination: HostId,
    payload: String
}

impl Message for StringMessage {
    fn source(&self) -> HostId {
        self.source.clone()
    }

    fn destination(&self) -> HostId {
        self.destination.clone()
    }

    fn payload(&self) -> [u8; 100] {
        let mut bytes = [0; 100];
        bytes[0] = self.source.0 as u8;
        bytes[1] = self.destination.0 as u8;

        for (index, character) in self.payload.chars().take(98).enumerate() {
            bytes[index + 2] = character as u8;
        }

        bytes
    }
}

impl StringMessage {
    pub fn new(source: HostId, destination: HostId, payload: String) -> Self {
        StringMessage { source, destination, payload }
    }

    pub fn from_bytes(bytes: [u8; 100]) -> Self {
        StringMessage {
            source: HostId(bytes[0] as char),
            destination: HostId(bytes[1] as char),
            payload: String::from_utf8(bytes[2..].to_vec()).unwrap()
        }
    }
}
