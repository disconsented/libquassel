use crate::Deserialize;
use crate::Serialize;

/// The first few bytes sent to the core to initialize the connection and setup if we want to use tls and compression
pub struct Init {
    pub tls: bool,
    pub compression: bool,
}

impl Init {
    pub fn new() -> Self {
        Self {
            tls: false,
            compression: false,
        }
    }

    pub fn compression(mut self, v: bool) {
        self.compression = v
    }

    pub fn tls(mut self, v: bool) {
        self.tls = v
    }

    pub fn serialize(self) -> Vec<u8> {
        // The handshake message
        let mut handshake: u32 = 0x42b33f00;

        // If TLS is enabled set the TLS bit on the handshake
        if self.tls {
            handshake |= 0x01;
        }

        // If COMPRESSION is enabled set the COMPRESSION bit on the handshake
        if self.compression {
            handshake |= 0x02;
        }

        // Select Protocol 2: Datastream

        let mut init: Vec<u8> = vec![];

        // Add handshake and protocol to our buffer
        init.extend(handshake.serialize().unwrap());
        init.extend(crate::message::Protocol::Datastream.serialize());

        return init;
    }

    pub fn parse(buf: &[u8]) -> Self {
        let (_, handshake) = u32::parse(&buf[0..4]).unwrap();

        let mut init = Self {
            tls: false,
            compression: false,
        };

        if (handshake & 0x01) >= 1 {
            init.tls = true
        }

        if (handshake & 0x02) >= 1 {
            init.tls = true
        }

        return init;
    }
}
